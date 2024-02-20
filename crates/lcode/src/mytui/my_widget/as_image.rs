use std::sync::mpsc::Sender;

use ratatui::prelude::*;
use ratatui_image::{protocol::StatefulProtocol, Resize};

/// A widget that uses a custom `ThreadProtocol` as state to offload resizing and encoding to a
/// background thread.
#[derive(Debug)]
pub struct ThreadImage {
    resize: Resize,
}

impl Default for ThreadImage {
    fn default() -> Self {
        Self::new()
    }
}

impl ThreadImage {
    pub const fn new() -> Self {
        Self { resize: Resize::Fit }
    }

    pub const fn resize(mut self, resize: Resize) -> Self {
        self.resize = resize;
        self
    }
}

impl StatefulWidget for ThreadImage {
    type State = ThreadProtocol;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.inner = match state.inner.take() {
            // We have the `protocol` and should either resize or render.
            Some(mut protocol) => {
                // If it needs resizing (grow or shrink) then send it away instead of rendering.
                if let Some(rect) = protocol.needs_resize(&self.resize, area) {
                    state
                        .tx
                        .send((protocol, self.resize, rect))
                        .ok();
                    None
                }
                else {
                    protocol.render(area, buf);
                    Some(protocol)
                }
            },
            // We are waiting to get back the protocol.
            None => None,
        };
    }
}

/// The state of a `ThreadImage`.
///
/// Has `inner` [`ResizeProtocol`] that is sent off to the `tx` mspc channel to do the
/// `resize_encode()` work.
pub struct ThreadProtocol {
    inner: Option<Box<dyn StatefulProtocol>>,
    tx:    Sender<(Box<dyn StatefulProtocol>, Resize, Rect)>,
}

impl ThreadProtocol {
    pub fn new(
        tx: Sender<(Box<dyn StatefulProtocol>, Resize, Rect)>,
        inner: Box<dyn StatefulProtocol>,
    ) -> Self {
        Self { inner: Some(inner), tx }
    }
}
