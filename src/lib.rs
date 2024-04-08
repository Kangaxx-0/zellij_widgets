//! # Zellij_widgets
//!
//! Zellij_widgets is a pure-rust library designed for creating rich plugins in [zellij](https://zellij.dev/),
//! Leveraging the Wasmer runtime.
//!
//! This crate combines the [ratatui](https://docs.rs/ratatui/latest/ratatui/index.html) and [crossterm](https://docs.rs/crossterm/latest/crossterm/index.html)
//! to provide a comprehensive set of widgets and utilities for building zellij plugin applications.
//!
//! [EXAMPLES]: https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples
//!  - [Block] : https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples/showcase_block
//!  - [list] : https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples/showcase_list
//!  - [tab] : https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples/showcase_tabs
//!  - [scrollbar] : https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples/showcase_scrollbar
//!  - [popup] : https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples/showcase_popup
//!  - [Custom component] : https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples/showcase_custom_button
//!
//! For an end-to-end demonstration, check out the [session_manager](https://github.com/Kangaxx-0/zellij_widgets/tree/main/examples/e2e/session_manager) example,
//! which provides a complete rewrite of an existing zellij plugin using `zellij_widgets`.
//!
//! ## NOTE :
//! - Like the [ratatui](https://docs.rs/ratatui/latest/ratatui/index.html) crate, zellij_widgets is also based on immediate mode GUI, that being said, for each frame, you need to re-render the entire UI.
//! For most cases, use zellij [update](https://docs.rs/zellij-tile/latest/zellij_tile/trait.ZellijPlugin.html#method.update) is the best way.

mod buffer;
mod frame;
mod plugin_pane;
mod widget;

pub mod core;
pub mod layout;
pub mod style;
pub mod text;
pub mod title;
pub mod uis;

pub(crate) mod test;

// prelude
pub mod prelude;
