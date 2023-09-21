use cxx::SharedPtr;
use parking_lot::Mutex;
use std::sync::Arc;
use webrtc_sys::frame_cryptor::{self as sys_fc};

use crate::peer_connection_factory::PeerConnectionFactory;
use crate::{rtp_receiver::RtpReceiver, rtp_sender::RtpSender};

pub type OnStateChange = Box<dyn FnMut(String, EncryptionState) + Send + Sync>;

#[derive(Debug, Clone)]
pub struct KeyProviderOptions {
    pub shared_key: bool,
    pub ratchet_window_size: i32,
    pub ratchet_salt: Vec<u8>,
    pub failure_tolerance: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    AesGcm,
    AesCbc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionState {
    New,
    Ok,
    EncryptionFailed,
    DecryptionFailed,
    MissingKey,
    KeyRatcheted,
    InternalError,
}

#[derive(Clone)]
pub struct KeyProvider {
    pub(crate) sys_handle: SharedPtr<sys_fc::ffi::KeyProvider>,
}

impl KeyProvider {
    pub fn new(options: KeyProviderOptions) -> Self {
        Self {
            sys_handle: sys_fc::ffi::new_key_provider(options.into()),
        }
    }

    pub fn set_shared_key(&self, key_index: i32, key: Vec<u8>) -> bool {
        self.sys_handle.set_shared_key(key_index, key)
    }

    pub fn ratchet_shared_key(&self, key_index: i32) -> Option<Vec<u8>> {
        self.sys_handle.ratchet_shared_key(key_index).ok()
    }

    pub fn get_shared_key(&self, key_index: i32) -> Option<Vec<u8>> {
        self.sys_handle.get_shared_key(key_index).ok()
    }

    pub fn set_key(&self, participant_id: String, key_index: i32, key: Vec<u8>) -> bool {
        self.sys_handle.set_key(participant_id, key_index, key)
    }

    pub fn ratchet_key(&self, participant_id: String, key_index: i32) -> Option<Vec<u8>> {
        self.sys_handle.ratchet_key(participant_id, key_index).ok()
    }

    pub fn get_key(&self, participant_id: String, key_index: i32) -> Option<Vec<u8>> {
        self.sys_handle.get_key(participant_id, key_index).ok()
    }

    pub fn set_sif_trailer(&self, trailer: Vec<u8>) {
        self.sys_handle.set_sif_trailer(trailer);
    }
}

#[derive(Clone)]
pub struct FrameCryptor {
    observer: Arc<RtcFrameCryptorObserver>,
    pub(crate) sys_handle: SharedPtr<sys_fc::ffi::FrameCryptor>,
}

impl FrameCryptor {
    pub fn new_for_rtp_sender(
        peer_factory: &PeerConnectionFactory,
        participant_id: String,
        algorithm: EncryptionAlgorithm,
        key_provider: KeyProvider,
        sender: RtpSender,
    ) -> Self {
        let observer = Arc::new(RtcFrameCryptorObserver::default());
        let sys_handle = sys_fc::ffi::new_frame_cryptor_for_rtp_sender(
            peer_factory.handle.sys_handle.clone(),
            participant_id,
            algorithm.into(),
            key_provider.sys_handle,
            sender.handle.sys_handle,
        );
        let fc = Self {
            observer: observer.clone(),
            sys_handle: sys_handle.clone(),
        };
        fc.sys_handle
            .register_observer(Box::new(sys_fc::RtcFrameCryptorObserverWrapper::new(
                observer,
            )));
        fc
    }

    pub fn new_for_rtp_receiver(
        peer_factory: &PeerConnectionFactory,
        participant_id: String,
        algorithm: EncryptionAlgorithm,
        key_provider: KeyProvider,
        receiver: RtpReceiver,
    ) -> Self {
        let observer = Arc::new(RtcFrameCryptorObserver::default());
        let sys_handle = sys_fc::ffi::new_frame_cryptor_for_rtp_receiver(
            peer_factory.handle.sys_handle.clone(),
            participant_id,
            algorithm.into(),
            key_provider.sys_handle,
            receiver.handle.sys_handle,
        );
        let fc = Self {
            observer: observer.clone(),
            sys_handle: sys_handle.clone(),
        };
        fc.sys_handle
            .register_observer(Box::new(sys_fc::RtcFrameCryptorObserverWrapper::new(
                observer,
            )));
        fc
    }

    pub fn set_enabled(self: &FrameCryptor, enabled: bool) {
        self.sys_handle.set_enabled(enabled);
    }

    pub fn enabled(self: &FrameCryptor) -> bool {
        self.sys_handle.enabled()
    }

    pub fn set_key_index(self: &FrameCryptor, index: i32) {
        self.sys_handle.set_key_index(index);
    }

    pub fn key_index(self: &FrameCryptor) -> i32 {
        self.sys_handle.key_index()
    }

    pub fn participant_id(self: &FrameCryptor) -> String {
        self.sys_handle.participant_id()
    }

    pub fn on_state_change(&self, handler: Option<OnStateChange>) {
        *self.observer.state_change_handler.lock() = handler;
    }
}

#[derive(Default)]
struct RtcFrameCryptorObserver {
    state_change_handler: Mutex<Option<OnStateChange>>,
}

impl sys_fc::RtcFrameCryptorObserver for RtcFrameCryptorObserver {
    fn on_frame_cryption_state_change(
        &self,
        participant_id: String,
        state: sys_fc::ffi::FrameCryptionState,
    ) {
        let mut handler = self.state_change_handler.lock();
        if let Some(f) = handler.as_mut() {
            f(participant_id, state.into());
        }
    }
}

impl From<sys_fc::ffi::Algorithm> for EncryptionAlgorithm {
    fn from(value: sys_fc::ffi::Algorithm) -> Self {
        match value {
            sys_fc::ffi::Algorithm::AesGcm => Self::AesGcm,
            sys_fc::ffi::Algorithm::AesCbc => Self::AesCbc,
            _ => panic!("unknown frame cyrptor Algorithm"),
        }
    }
}

impl From<EncryptionAlgorithm> for sys_fc::ffi::Algorithm {
    fn from(value: EncryptionAlgorithm) -> Self {
        match value {
            EncryptionAlgorithm::AesGcm => Self::AesGcm,
            EncryptionAlgorithm::AesCbc => Self::AesCbc,
        }
    }
}

impl From<sys_fc::ffi::FrameCryptionState> for EncryptionState {
    fn from(value: sys_fc::ffi::FrameCryptionState) -> Self {
        match value {
            sys_fc::ffi::FrameCryptionState::New => Self::New,
            sys_fc::ffi::FrameCryptionState::Ok => Self::Ok,
            sys_fc::ffi::FrameCryptionState::EncryptionFailed => Self::EncryptionFailed,
            sys_fc::ffi::FrameCryptionState::DecryptionFailed => Self::DecryptionFailed,
            sys_fc::ffi::FrameCryptionState::MissingKey => Self::MissingKey,
            sys_fc::ffi::FrameCryptionState::KeyRatcheted => Self::KeyRatcheted,
            sys_fc::ffi::FrameCryptionState::InternalError => Self::InternalError,
            _ => panic!("unknown frame cyrptor FrameCryptionState"),
        }
    }
}

impl From<KeyProviderOptions> for sys_fc::ffi::KeyProviderOptions {
    fn from(value: KeyProviderOptions) -> Self {
        Self {
            shared_key: value.shared_key,
            ratchet_window_size: value.ratchet_window_size,
            ratchet_salt: value.ratchet_salt,
            failure_tolerance: value.failure_tolerance,
        }
    }
}
