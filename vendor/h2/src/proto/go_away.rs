use codec::Codec;
use frame::{self, Reason, StreamId};

use bytes::Buf;
use futures::{Async, Poll};
use std::io;
use tokio_io::AsyncWrite;

/// Manages our sending of GOAWAY frames.
#[derive(Debug)]
pub(super) struct GoAway {
    /// Whether the connection should close now, or wait until idle.
    close_now: bool,
    /// Records if we've sent any GOAWAY before.
    going_away: Option<GoingAway>,
    /// Whether the user started the GOAWAY by calling `abrupt_shutdown`.
    is_user_initiated: bool,
    /// A GOAWAY frame that must be buffered in the Codec immediately.
    pending: Option<frame::GoAway>,
}

/// Keeps a memory of any GOAWAY frames we've sent before.
///
/// This looks very similar to a `frame::GoAway`, but is a separate type. Why?
/// Mostly for documentation purposes. This type is to record status. If it
/// were a `frame::GoAway`, it might appear like we eventually wanted to
/// serialize it. We **only** want to be able to look up these fields at a
/// later time.
///
/// (Technically, `frame::GoAway` should gain an opaque_debug_data field as
/// well, and we wouldn't want to save that here to accidentally dump in logs,
/// or waste struct space.)
#[derive(Debug)]
struct GoingAway {
    /// Stores the highest stream ID of a GOAWAY that has been sent.
    ///
    /// It's illegal to send a subsequent GOAWAY with a higher ID.
    last_processed_id: StreamId,

    /// Records the error code of any GOAWAY frame sent.
    reason: Reason,
}

impl GoAway {
    pub fn new() -> Self {
        GoAway {
            close_now: false,
            going_away: None,
            is_user_initiated: false,
            pending: None,
        }
    }

    /// Enqueue a GOAWAY frame to be written.
    ///
    /// The connection is expected to continue to run until idle.
    pub fn go_away(&mut self, f: frame::GoAway) {
        if let Some(ref going_away) = self.going_away {
            assert!(
                f.last_stream_id() <= going_away.last_processed_id,
                "GOAWAY stream IDs shouldn't be higher; \
                last_processed_id = {:?}, f.last_stream_id() = {:?}",
                going_away.last_processed_id,
                f.last_stream_id(),
            );
        }

        self.going_away = Some(GoingAway {
            last_processed_id: f.last_stream_id(),
            reason: f.reason(),
        });
        self.pending = Some(f);
    }

    pub fn go_away_now(&mut self, f: frame::GoAway) {
        self.close_now = true;
        if let Some(ref going_away) = self.going_away {
            // Prevent sending the same GOAWAY twice.
            if going_away.last_processed_id == f.last_stream_id()
                && going_away.reason == f.reason() {
                return;
            }
        }
        self.go_away(f);
    }

    pub fn go_away_from_user(&mut self, f: frame::GoAway) {
        self.is_user_initiated = true;
        self.go_away_now(f);
    }

    /// Return if a GOAWAY has ever been scheduled.
    pub fn is_going_away(&self) -> bool {
        self.going_away.is_some()
    }

    pub fn is_user_initiated(&self) -> bool {
        self.is_user_initiated
    }

    /// Return the last Reason we've sent.
    pub fn going_away_reason(&self) -> Option<Reason> {
        self.going_away
            .as_ref()
            .map(|g| g.reason)
    }

    /// Returns if the connection should close now, or wait until idle.
    pub fn should_close_now(&self) -> bool {
        self.pending.is_none() && self.close_now
    }

    /// Returns if the connection should be closed when idle.
    pub fn should_close_on_idle(&self) -> bool {
        !self.close_now && self.going_away
            .as_ref()
            .map(|g| g.last_processed_id != StreamId::MAX)
            .unwrap_or(false)
    }

    /// Try to write a pending GOAWAY frame to the buffer.
    ///
    /// If a frame is written, the `Reason` of the GOAWAY is returned.
    pub fn send_pending_go_away<T, B>(&mut self, dst: &mut Codec<T, B>) -> Poll<Option<Reason>, io::Error>
    where
        T: AsyncWrite,
        B: Buf,
    {
        if let Some(frame) = self.pending.take() {
            if !dst.poll_ready()?.is_ready() {
                self.pending = Some(frame);
                return Ok(Async::NotReady);
            }

            let reason = frame.reason();
            dst.buffer(frame.into())
                .ok()
                .expect("invalid GOAWAY frame");

            return Ok(Async::Ready(Some(reason)));
        } else if self.should_close_now() {
            return Ok(Async::Ready(self.going_away_reason()));
        }

        Ok(Async::Ready(None))
    }
}
