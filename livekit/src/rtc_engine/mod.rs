use crate::options::TrackPublishOptions;
use crate::prelude::LocalTrack;
use crate::rtc_engine::lk_runtime::LkRuntime;
use crate::rtc_engine::rtc_session::{RtcSession, SessionEvent, SessionEvents};
use crate::DataPacketKind;
use livekit_api::signal_client::{SignalError, SignalOptions};
use livekit_protocol as proto;
use livekit_webrtc::prelude::*;
use livekit_webrtc::session_description::SdpParseError;
use parking_lot::Mutex;
use std::fmt::Debug;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::RwLock as AsyncRwLock;
use tokio::sync::{mpsc, oneshot};
use tokio::sync::{Mutex as AsyncMutex, Notify};
use tokio::task::JoinHandle;
use tokio::time::{interval, Interval, MissedTickBehavior};

pub mod lk_runtime;
mod peer_transport;
mod rtc_events;
mod rtc_session;

pub(crate) type EngineEmitter = mpsc::Sender<EngineEvent>;
pub(crate) type EngineEvents = mpsc::Receiver<EngineEvent>;
pub(crate) type EngineResult<T> = Result<T, EngineError>;

#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum SimulateScenario {
    SignalReconnect,
    Speaker,
    NodeFailure,
    ServerLeave,
    Migration,
    ForceTcp,
    ForceTls,
}

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("signal failure: {0}")]
    Signal(#[from] SignalError),
    #[error("internal webrtc failure")]
    Rtc(#[from] RtcError),
    #[error("failed to parse sdp")]
    Parse(#[from] SdpParseError),
    #[error("serde error")]
    Serde(#[from] serde_json::Error),
    #[error("failed to send data to the datachannel")]
    Data(#[from] DataChannelError),
    #[error("connection error: {0}")]
    Connection(String),
    #[error("decode error")]
    Decode(#[from] prost::DecodeError),
    #[error("internal error: {0}")]
    Internal(String), // Unexpected error
}

#[derive(Debug)]
pub enum EngineEvent {
    ParticipantUpdate {
        updates: Vec<proto::ParticipantInfo>,
    },
    MediaTrack {
        track: MediaStreamTrack,
        stream: MediaStream,
        receiver: RtpReceiver,
    },
    Data {
        participant_sid: String,
        payload: Vec<u8>,
        kind: DataPacketKind,
    },
    SpeakersChanged {
        speakers: Vec<proto::SpeakerInfo>,
    },
    ConnectionQuality {
        updates: Vec<proto::ConnectionQualityInfo>,
    },
    Resuming,
    Resumed,
    Restarting,
    Restarted,
    Disconnected,
}

pub const RECONNECT_ATTEMPTS: u32 = 10;
pub const RECONNECT_INTERVAL: Duration = Duration::from_secs(5);

/// Represents a running RTCSession with the ability to close the session
/// and the engine_task
#[derive(Debug)]
struct EngineHandle {
    session: RtcSession,
    engine_task: JoinHandle<()>,
    close_sender: oneshot::Sender<()>,
}

struct EngineInner {
    // Keep a strong reference to LkRuntime to avoid creating a new RtcRuntime or PeerConnection factory accross multiple Rtc sessions
    #[allow(dead_code)]
    lk_runtime: Arc<LkRuntime>,
    engine_emitter: EngineEmitter,

    // Last/current session JoinResponse
    // We keep a clone of the join response here because the room needs it
    // (directly accessing the running_handle requires an async context to lock the Mutex and a getter needs a short lock)
    // Maybe there is a better way to do it?
    join_response: Mutex<proto::JoinResponse>,
    running_handle: AsyncRwLock<Option<EngineHandle>>,

    // Reconnecting fields
    closed: AtomicBool, // True if closed or the reconnection failed (Note that this is false when reconnecting or resuming)
    reconnecting: AtomicBool,
    full_reconnect: AtomicBool, // If true, the next reconnect attempt will skip resume and directly try a full reconnect
    reconnect_interval: AsyncMutex<Interval>,
    reconnect_notifier: Arc<Notify>, // Called when the reconnection task finisehd, successful or not
}

impl Debug for EngineInner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("EngineInner")
            .field("closed", &self.closed)
            .field("reconnecting", &self.reconnecting)
            .field("full_reconnect", &self.full_reconnect)
            .finish()
    }
}

#[derive(Debug)]
pub struct RtcEngine {
    inner: Arc<EngineInner>,
}

impl RtcEngine {
    pub async fn connect(
        url: &str,
        token: &str,
        options: SignalOptions,
    ) -> EngineResult<(Self, EngineEvents)> {
        let (engine_emitter, engine_events) = mpsc::channel(8);

        let mut reconnect_interval = interval(RECONNECT_INTERVAL);
        reconnect_interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

        let inner = Arc::new(EngineInner {
            lk_runtime: LkRuntime::instance(),
            running_handle: Default::default(),
            engine_emitter,
            join_response: Default::default(), // Will directly be replaced by the connect method below
            closed: Default::default(),
            reconnecting: Default::default(),
            full_reconnect: Default::default(),
            reconnect_interval: AsyncMutex::new(reconnect_interval),
            reconnect_notifier: Arc::new(Notify::new()),
        });

        inner.connect(url, token, options).await?;
        Ok((Self { inner }, engine_events))
    }

    pub async fn close(&self) {
        self.inner.close().await
    }

    pub async fn publish_data(
        &self,
        data: &proto::DataPacket,
        kind: DataPacketKind,
    ) -> EngineResult<()> {
        // Make sure we are connected before trying to send data
        self.inner.wait_reconnection().await?;
        let handle = self.inner.running_handle.read().await;
        let session = &handle.as_ref().unwrap().session;
        session.publish_data(data, kind).await
    }

    pub async fn simulate_scenario(&self, scenario: SimulateScenario) -> EngineResult<()> {
        self.inner.wait_reconnection().await?;
        let handle = self.inner.running_handle.read().await;
        let session = &handle.as_ref().unwrap().session;
        session.simulate_scenario(scenario).await;
        Ok(())
    }

    pub async fn add_track(&self, req: proto::AddTrackRequest) -> EngineResult<proto::TrackInfo> {
        self.inner.wait_reconnection().await?;
        let handle = self.inner.running_handle.read().await;
        let session = &handle.as_ref().unwrap().session;
        session.add_track(req).await
    }

    pub async fn remove_track(&self, sender: RtpSender) -> EngineResult<()> {
        self.inner.wait_reconnection().await?;
        let handle = self.inner.running_handle.read().await;
        let session = &handle.as_ref().unwrap().session;
        session.remove_track(sender).await
    }

    pub async fn create_sender(
        &self,
        track: LocalTrack,
        options: TrackPublishOptions,
        encodings: Vec<RtpEncodingParameters>,
    ) -> EngineResult<RtpTransceiver> {
        self.inner.wait_reconnection().await?;
        let handle = self.inner.running_handle.read().await;
        let session = &handle.as_ref().unwrap().session;
        session.create_sender(track, options, encodings).await
    }

    pub async fn negotiate_publisher(&self) -> EngineResult<()> {
        // TODO(theomonnom): guard for reconnection
        self.inner.wait_reconnection().await?;
        let handle = self.inner.running_handle.read().await;
        let session = &handle.as_ref().unwrap().session;
        session.negotiate_publisher().await
    }

    pub async fn send_request(&self, msg: proto::signal_request::Message) -> EngineResult<()> {
        if self.inner.reconnecting.load(Ordering::Acquire) {
            // When doing a full reconnect, it is safe to ignore the messages, we don't wait for reconnection here
            return Ok(()); // TODO(theomonnom): Maybe we should still return an error instead?
        }

        let handle = self.inner.running_handle.read().await;
        let session = &handle.as_ref().unwrap().session; // Unwrap should be OK here (running_handle is always valid when not reconnecting)
        session.signal_client().send(msg).await;
        Ok(())
    }

    pub fn join_response(&self) -> proto::JoinResponse {
        self.inner.join_response.lock().clone()
    }
}

impl EngineInner {
    async fn engine_task(
        self: Arc<Self>,
        mut session_events: SessionEvents,
        mut close_receiver: oneshot::Receiver<()>,
    ) {
        loop {
            tokio::select! {
                res = session_events.recv() => {
                    if let Some(event) = res {
                        if let Err(err) = self.on_session_event(event).await {
                            log::error!("failed to handle session event: {:?}", err);
                        }
                    }
                },
                 _ = &mut close_receiver => {
                    log::trace!("closing engine task");
                    break;
                }
            }
        }
    }

    async fn on_session_event(self: &Arc<Self>, event: SessionEvent) -> EngineResult<()> {
        match event {
            SessionEvent::Close {
                source,
                reason,
                can_reconnect,
                retry_now,
                full_reconnect,
            } => {
                log::info!("received session close: {}, {:?}", source, reason);
                if can_reconnect {
                    self.try_reconnect(retry_now, full_reconnect);
                } else {
                    // Spawning a new task because the close function wait for the engine_task to
                    // finish. (Where this function is called from)
                    tokio::spawn({
                        let inner = self.clone();
                        async move {
                            inner.close().await;
                        }
                    });
                }
            }
            SessionEvent::Data {
                participant_sid,
                payload,
                kind,
            } => {
                let _ = self
                    .engine_emitter
                    .send(EngineEvent::Data {
                        participant_sid,
                        payload,
                        kind,
                    })
                    .await;
            }
            SessionEvent::MediaTrack {
                track,
                stream,
                receiver,
            } => {
                let _ = self
                    .engine_emitter
                    .send(EngineEvent::MediaTrack {
                        track,
                        stream,
                        receiver,
                    })
                    .await;
            }
            SessionEvent::ParticipantUpdate { updates } => {
                let _ = self
                    .engine_emitter
                    .send(EngineEvent::ParticipantUpdate { updates })
                    .await;
            }
            SessionEvent::SpeakersChanged { speakers } => {
                let _ = self
                    .engine_emitter
                    .send(EngineEvent::SpeakersChanged { speakers })
                    .await;
            }
            SessionEvent::ConnectionQuality { updates } => {
                let _ = self
                    .engine_emitter
                    .send(EngineEvent::ConnectionQuality { updates })
                    .await;
            }
            SessionEvent::Connected => {}
        }
        Ok(())
    }

    async fn connect(
        self: &Arc<Self>,
        url: &str,
        token: &str,
        options: SignalOptions,
    ) -> EngineResult<()> {
        let mut running_handle = self.running_handle.write().await;

        let (session, join_response, session_events) =
            RtcSession::connect(url, token, options).await?;

        let (close_sender, close_receiver) = oneshot::channel();
        let engine_task = tokio::spawn(self.clone().engine_task(session_events, close_receiver));

        let engine_handle = EngineHandle {
            session,
            engine_task,
            close_sender,
        };

        // Always update the join response after a new session is created (first session or full reconnect)
        *self.join_response.lock() = join_response;
        *running_handle = Some(engine_handle);
        Ok(())
    }

    async fn terminate_session(&self) {
        if let Some(handle) = self.running_handle.write().await.take() {
            handle.session.close().await;
            let _ = handle.close_sender.send(());
            let _ = handle.engine_task.await;
        }
    }

    async fn close(&self) {
        self.closed.store(true, Ordering::Release);
        self.terminate_session().await;
        let _ = self.engine_emitter.send(EngineEvent::Disconnected).await;
    }

    // Wait for the reconnection task to finish
    // Return directly if no open RTCSession
    async fn wait_reconnection(&self) -> EngineResult<()> {
        if self.closed.load(Ordering::SeqCst) {
            Err(EngineError::Connection("engine is closed".to_owned()))?
        }

        if self.reconnecting.load(Ordering::Acquire) {
            // If currently reconnecting, wait for the reconnect task to finish
            self.reconnect_notifier.notified().await;
        }

        // reconnect_task is finished here, so it is fine to try to read the RwLock here (should be a short lock)
        // (the reconnection logic can lock the running_handle for a long time, e.g when resuming)

        if self.running_handle.read().await.is_none() {
            Err(EngineError::Connection("reconnection failed".to_owned()))?
        }

        Ok(())
    }

    /// Start the reconnect task if not already started
    /// Ask to retry directly if `retry_now` is true
    /// Ask for a full reconnect if `full_reconnect` is true
    fn try_reconnect(self: &Arc<Self>, retry_now: bool, full_reconnect: bool) {
        if self.closed.load(Ordering::Acquire) {
            return;
        }

        if self.reconnecting.load(Ordering::SeqCst) {
            let inner = self.clone();
            if retry_now {
                tokio::spawn(async move {
                    inner.reconnect_interval.lock().await.reset(); // Retry directly
                });
                self.full_reconnect.store(full_reconnect, Ordering::Release);
            }
            return;
        }

        log::warn!("reconnecting RTCEngine...");

        tokio::spawn({
            let inner = self.clone();
            async move {
                // Reconnetion logic
                inner.reconnect_interval.lock().await.reset(); // Retry directly
                inner.reconnecting.store(true, Ordering::Release);
                inner
                    .full_reconnect
                    .store(full_reconnect, Ordering::Release);

                let res = inner.reconnect_task().await; // Wait for the reconnection task to finish
                inner.reconnecting.store(false, Ordering::Release);

                if res.is_ok() {
                    log::warn!("RTCEngine successfully reconnected")
                } else {
                    log::error!("failed to reconnect after {} attemps", RECONNECT_ATTEMPTS);
                    inner.close().await;
                }

                inner.reconnect_notifier.notify_waiters();
            }
        });
    }

    /// Runned every time the PeerConnection or the SignalClient is closed
    /// We first try to resume the connection, if it fails, we start a full reconnect.
    async fn reconnect_task(self: &Arc<Self>) -> EngineResult<()> {
        // Get the latest connection info from the signal_client (including the refreshed token because the initial join token may have expired)
        let running_handle = self.running_handle.read().await;
        let signal_client = running_handle.as_ref().unwrap().session.signal_client();
        let url = signal_client.url();
        let token = signal_client.token();
        let options = signal_client.options();
        drop(running_handle);

        for i in 0..RECONNECT_ATTEMPTS {
            if self.closed.load(Ordering::Acquire) {
                // The user closed the RTCEngine, cancel the reconnection task
                return Ok(());
            }

            if self.full_reconnect.load(Ordering::SeqCst) {
                if i == 0 {
                    let _ = self.engine_emitter.send(EngineEvent::Restarting).await;
                }

                log::info!("restarting connection... attempt: {}", i);
                if let Err(err) = self
                    .try_restart_connection(&url, &token, options.clone())
                    .await
                {
                    log::error!("restarting connection failed: {}", err);
                } else {
                    let _ = self.engine_emitter.send(EngineEvent::Restarted).await;
                    return Ok(());
                }
            } else {
                if i == 0 {
                    let _ = self.engine_emitter.send(EngineEvent::Resuming).await;
                }

                log::info!("resuming connection... attempt: {}", i);
                if let Err(err) = self.try_resume_connection().await {
                    log::error!("resuming connection failed: {}", err);
                    if let EngineError::Signal(_) = err {
                        self.full_reconnect.store(true, Ordering::SeqCst);
                    }
                } else {
                    let _ = self.engine_emitter.send(EngineEvent::Resumed).await;
                    return Ok(());
                }
            }

            self.reconnect_interval.lock().await.tick().await;
        }

        Err(EngineError::Connection("failed to reconnect".to_owned()))
    }

    /// Try to recover the connection by doing a full reconnect.
    /// It recreates a new RtcSession
    async fn try_restart_connection(
        self: &Arc<Self>,
        url: &str,
        token: &str,
        options: SignalOptions,
    ) -> EngineResult<()> {
        self.terminate_session().await;
        self.connect(url, token, options).await?;

        let handle = self.running_handle.read().await;
        let session = &handle.as_ref().unwrap().session;
        session.wait_pc_connection().await
    }

    /// Try to restart the current session
    async fn try_resume_connection(&self) -> EngineResult<()> {
        let handle = self.running_handle.read().await;
        let session = &handle.as_ref().unwrap().session;
        session.restart().await?;
        session.wait_pc_connection().await
    }
}
