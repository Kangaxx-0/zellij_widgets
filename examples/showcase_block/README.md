## How to use it
Update your zellij config to bind the key

```
    session {
        bind "t" {
            LaunchOrFocusPlugin "file:<local_path>/zellij_widgets/examples/showcase_block/target/wasm32-wasi/debug/showcase_block.wasm" {
                floating true
                move_to_focused_tab true
            };
            SwitchToMode "Normal"
        }
    }
```

When you have the zellij running, hit the short key for the session with `t`, once you see the panel, hit the key `c`, and you should see blocks
