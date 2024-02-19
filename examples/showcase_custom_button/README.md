This sample code provides a basic overview of how to use this crate to build a plugin system for Zellij. The key concept to understand is that it is your responsibility to manage the plugin state for rendering content through the WASM runtime. If you have experience with Ratatui, the primary distinction lies in this aspect of state management.

## How to use it
Update your zellij config to bind the key

```
    session {
        bind "t" {
            LaunchOrFocusPlugin "file:<local_path>/zellij_widgets/examples/showcase_custom_button/target/wasm32-wasi/debug/showcase_custom_button.wasm" {
                floating true
                move_to_focused_tab true
            };
            SwitchToMode "Normal"
        }
    }
```

When you have the zellij running, hit the short key for the session with `t`, once you see the panel, hit the key `c`, and you should see the buttons

