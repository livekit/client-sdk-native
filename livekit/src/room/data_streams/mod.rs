use std::{
    pin::Pin,
    task::{Context, Poll},
};

use livekit_runtime::Stream;
use tokio::sync::mpsc;

pub struct DataStreamChunk {
    pub stream_id: String,
    pub chunk_index: u64,
    pub content: Vec<u8>,
    pub complete: bool,
    pub version: i32,
}

pub struct FileStreamInfo {
    pub stream_id: String,
    pub timestamp: i64,
    pub topic: String,
    pub mime_type: String,
    pub total_length: Option<u64>,
    pub total_chunks: Option<u64>,
}

pub struct FileStreamReader {
    update_rx: mpsc::UnboundedReceiver<DataStreamChunk>,
    pub info: FileStreamInfo,
    is_closed: bool,
}

impl FileStreamReader {
    pub fn new(info: FileStreamInfo) -> (Self, FileStreamUpdater) {
        let (update_tx, update_rx) = mpsc::unbounded_channel();
        (Self { update_rx, info, is_closed: false }, FileStreamUpdater { update_tx })
    }

    fn close(&mut self) {
        self.is_closed = true;
        self.update_rx.close();
    }
}

impl Drop for FileStreamReader {
    fn drop(&mut self) {
        self.close();
    }
}

impl Stream for FileStreamReader {
    type Item = Vec<u8>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.is_closed {
            return Poll::Ready(None); // Stream is closed‚, stop yielding updates
        }
        match self.update_rx.poll_recv(cx) {
            Poll::Ready(Some(update)) => {
                if update.complete {
                    Poll::Ready(None) // Close stream after receiving a complete update
                } else {
                    Poll::Ready(Some(update.content)) // Continue with data updates
                }
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Helper to send updates to the `FileStream`.
pub struct FileStreamUpdater {
    update_tx: mpsc::UnboundedSender<DataStreamChunk>,
}

impl FileStreamUpdater {
    /// Sends an update to the `FileStream`.
    pub fn send_update(
        &self,
        data: DataStreamChunk,
    ) -> Result<(), mpsc::error::SendError<DataStreamChunk>> {
        self.update_tx.send(data)
    }
}
