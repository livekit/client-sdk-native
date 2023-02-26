use crate::{imp::data_channel as dc_imp, rtp_parameters::Priority};
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct DataChannelInit {
    pub ordered: bool,
    pub has_max_retransmit_time: bool,
    pub max_retransmit_time: i32,
    pub has_max_retransmits: bool,
    pub max_retransmits: i32,
    pub protocol: String,
    pub negotiated: bool,
    pub id: i32,
    pub has_priority: bool,
    pub priority: Priority,
}

#[derive(Debug, Error)]
pub enum DataChannelError {
    #[error("failed to send data, dc not open? send buffer is full ?")]
    Send,
    #[error("only utf8 strings can be sent")]
    Utf8(#[from] Utf8Error),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataState {
    Connecting,
    Open,
    Closing,
    Closed,
}

#[derive(Debug)]
pub struct DataBuffer<'a> {
    pub data: &'a [u8],
    pub binary: bool,
}

pub type OnStateChange = Box<dyn FnMut(DataState) + Send + Sync>;
pub type OnMessage = Box<dyn FnMut(DataBuffer) + Send + Sync>;
pub type OnBufferedAmountChange = Box<dyn FnMut(u64) + Send + Sync>;

#[derive(Clone)]
pub struct DataChannel {
    handle: dc_imp::DataChannel,
}

impl DataChannel {
    pub fn send(&self, data: &[u8], binary: bool) -> Result<(), DataChannelError> {
        self.handle.send(data, binary)
    }

    pub fn label(&self) -> String {
        self.handle.label()
    }

    pub fn state(&self) -> DataState {
        self.handle.state()
    }

    pub fn close(&self) {
        self.handle.close()
    }

    pub fn on_state_change(&self, callback: Option<OnStateChange>) {
        self.handle.on_state_change(callback)
    }

    pub fn on_message(&self, callback: Option<OnMessage>) {
        self.handle.on_message(callback)
    }

    pub fn on_buffered_amount_change(&self, callback: Option<OnBufferedAmountChange>) {
        self.handle.on_buffered_amount_change(callback)
    }
}
