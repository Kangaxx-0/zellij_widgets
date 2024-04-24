# Zellij Widgets

Zellij Widgets provides a set of widgets for the Zellij plugin system. It is designed to run with [zellij](https://zellij.dev/)  and is a customized combination of the [crossterm](https://github.com/crossterm-rs/crossterm) and [ratatui](https://github.com/ratatui-org/ratatui) crates.

## Key Features
1. Reusable UI Components: Offers a collection of UI components that abstract away the complexity of dealing with ANSI codes, making it easier for Zellij plugin developers to create user interfaces.

![demo](./assets/images/showcase_e2e.gif)

## Prerequisites
To use Zellij Widgets, developers should have a basic understanding of the Zellij plugin system. Zellij plugins communicate with the Zellij host via the Wasmer runtime, and Zellij provides interfaces such as the ZellijPlugin crate.

## Useful Links
- [WASI_io](https://github.com/WebAssembly/wasi-io)
- [WASI Implementation](https://docs.rs/wasmer-wasi/latest/wasmer_wasi/)
  - [Wasi_state](https://docs.rs/wasmer-wasi/latest/wasmer_wasi/struct.WasiState.html)
  - [VirtualFile](https://docs.rs/wasmer-wasi/latest/wasmer_wasi/trait.VirtualFile.html)
  - [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html)
  - [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html)
  - [Seek](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html)

## Notes:
1. [zellij](https://github.com/zellij-org/zellij/blob/main/zellij-server/Cargo.toml#L22) is using old version of wasmer, don't get confused if you see the standard I/O steam interfaces are different from latest wasmer which leverages the power of the async from tokio

## Acknowledgments
This project reuses a significant amount of code from the crossterm and ratatui projects. I would like to express our gratitude to the authors and contributors of these projects for their excellent work. All credits are theirs.

## License
Zellij Widgets is licensed under the same terms as crossterm and ratatui. Please refer to their respective licenses for more details.


