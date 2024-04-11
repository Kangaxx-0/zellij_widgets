//! Provides the `Widget` and `StateWidget` traits, they define how widgets are rendered in the terminal.
use crate::{buffer::Buffer, layout::Geometry};

/// Essential trait for a User Interface (UI) component in Zellij.
///
/// This trait represents a widget, which is a UI component that can be rendered. Any UI component in Zellij must implement this trait. This is crucial because it's the only way the Zellij host can understand what the actual ANSI code is for the UI component.
///
/// The `render` method is the only method required to implement a custom widget. It draws the current state of the widget in the given buffer.
///
/// # Arguments
///
/// * `area` - The geometry of the widget's area.
/// * `buf` - The buffer to render the widget in.
///
/// # Example
///
/// ```
/// use zellij_widgets::prelude::{Buffer,Geometry,Widget};
///
/// struct MyWidget;
///
/// impl Widget for MyWidget {
///     fn render(self, area: Geometry, buf: &mut Buffer) {
///         // Implementation details for rendering the widget
///     }
/// }
/// ```
pub trait Widget {
    fn render(self, area: Geometry, buf: &mut Buffer);
}

/// Essential trait for a User Interface (UI) component in Zellij that has a state.
///
/// This is another trait that represents a widget, but it's for widgets that have a state.
///
/// # Arguments
/// * `state` - The State is not a mutable reference, so the widget can't change the state during render directly. Instead, state should be tracked by the zellij plugin state where you implement `ZellijPlugin` crate
pub trait StateWidget {
    type State;
    fn render(self, area: Geometry, buf: &mut Buffer, state: &mut Self::State);
}
