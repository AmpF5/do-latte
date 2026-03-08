use std::fmt::Debug;

use derive_setters::Setters;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::widgets::known_size::KnownSize;

/// The `KnownSizeWrapper` struct wraps a widget and provides a fixed size for it.
///
/// This struct is used to wrap a widget and provide a fixed size for it. This is useful when you
/// want to use a widget that does not implement [`KnownSize`] as the body of a popup.
#[derive(Debug, Setters)]
pub struct KnownSizeWrapper<W> {
    #[setters(skip)]
    pub inner: W,
    pub width: usize,
    pub height: usize,
}

impl<W: Widget> Widget for KnownSizeWrapper<W> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.inner.render(area, buf);
    }
}

impl<W> Widget for &KnownSizeWrapper<W>
where
    for<'a> &'a W: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.inner.render(area, buf);
    }
}

impl<W> KnownSize for KnownSizeWrapper<W> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl<W> KnownSize for &KnownSizeWrapper<W> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl<W> KnownSizeWrapper<W> {
    /// Create a new `KnownSizeWrapper` with the given widget and size.
    pub const fn new(inner: W, width: usize, height: usize) -> Self {
        Self {
            inner,
            width,
            height,
        }
    }
}
