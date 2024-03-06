use crate::prelude::*;

/// A struct representing the state of a Scrollbar widget.
///
/// # Important
///
/// It's essential to set the `content_length` field when using this struct. This field
/// represents the total length of the scrollable content. The default value is zero
/// which will result in the Scrollbar not rendering.
///
/// For example, in the following list, assume there are 4 bullet points:
///
/// - the `content_length` is 4
/// - the `position` is 0
///
/// ```text
/// ┌───────────────┐
/// │1. this is a   █
/// │   single item █
/// │2. this is a   ║
/// │   second item ║
/// └───────────────┘
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ScrollbarState {
    // The total length of the scrollable content.
    pub content_length: usize,
    // The current position within the scrollable content.
    pub position: usize,
}

impl ScrollbarState {
    /// Constructs a new ScrollbarState with the specified content length.
    pub fn new(content_length: usize) -> Self {
        Self {
            content_length,
            ..Default::default()
        }
    }
    /// Sets the scroll position of the scrollbar and returns the modified ScrollbarState.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn position(mut self, position: usize) -> Self {
        self.position = position;
        self
    }

    /// Sets the length of the scrollable content and returns the modified ScrollbarState.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn content_length(mut self, content_length: usize) -> Self {
        self.content_length = content_length;
        self
    }

    /// Decrements the scroll position by one, ensuring it doesn't go below zero.
    pub fn prev(&mut self) {
        self.position = self.position.saturating_sub(1);
    }

    /// Increments the scroll position by one, ensuring it doesn't exceed the length of the content.
    pub fn next(&mut self) {
        self.position = self
            .position
            .saturating_add(1)
            .clamp(0, self.content_length.saturating_sub(1))
    }

    /// Sets the scroll position to the start of the scrollable content.
    pub fn first(&mut self) {
        self.position = 0;
    }

    /// Sets the scroll position to the end of the scrollable content.
    pub fn last(&mut self) {
        self.position = self.content_length.saturating_sub(1)
    }

    /// Changes the scroll position based on the provided ScrollDirection.
    pub fn scroll(&mut self, direction: ScrollDirection) {
        match direction {
            ScrollDirection::Forward => {
                self.next();
            }
            ScrollDirection::Backward => {
                self.prev();
            }
        }
    }
}
