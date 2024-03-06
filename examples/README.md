# Exanmples

This folder contains unreleased code, but before you try any of them, you need to have a good key-binding mapping, such as 

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

Above sample config would allow you to interact with `custom_button` code, when you have plugin running, press `c`


## You need to call `render` on demand
Keep in mind, you are responsible to call [render](https://zellij.dev/documentation/plugin-lifecycle) function every time you want to redraw ui, there is no loop.


