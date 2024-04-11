//! # Zellij_widgets
//!
//! Zellij_widgets is a pure-rust library designed for creating rich plugins in [zellij](https://zellij.dev/),
//! Leveraging the Wasmer runtime.
//!
//! This crate combines the [ratatui](https://docs.rs/ratatui/latest/ratatui/index.html) and [crossterm](https://docs.rs/crossterm/latest/crossterm/index.html)
//! to provide a comprehensive set of widgets and utilities for building zellij plugin applications.
//!
//! ## Important Concepts
//! Zellij does not have a continuous rendering loop like some UI frameworks,
//! instead, rendering in Zellij typically occurs in response to user input or when the something has changes,
//! this means that designing animations or expecting continuous rendering for dynamic UI updates is not feasible with zellij_widgets.
//! Take a look at zellij's [update](https://docs.rs/zellij-tile/latest/zellij_tile/trait.ZellijPlugin.html#method.update) interface
//!
//! [EXAMPLES]: <https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples>
//!  - Block : <https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples/showcase_block>
//!  - List : <https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples/showcase_list>
//!  - Tab : <https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples/showcase_tabs>
//!  - Scrollbar : <https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples/showcase_scrollbar>
//!  - Popup : <https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples/showcase_popup>
//!  - Custom component : <https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples/showcase_custom_button>
//!
//! For an end-to-end demonstration, check out the [session_manager](https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples/e2e/session_manager)
//! example, which provides a complete rewrite of an existing zellij plugin using `zellij_widgets`.
//!

pub mod buffer;
pub mod core;
pub mod frame;
pub mod layout;
pub mod plugin_pane;
pub mod style;
pub mod text;
pub mod title;
pub mod uis;
pub mod widget;

pub(crate) mod test;

// prelude
pub mod prelude;
