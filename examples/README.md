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


## Come from `Ratatui`
Keep in mind, there is no loop controls rendering like `Ratatui`, its your responsibility to **implicitly** call [render](https://zellij.dev/documentation/plugin-lifecycle) function when you want to redraw ui every time

If you want to see more fundamental concepts in zellij, please refer to my another [repo](https://github.com/Kangaxx-0/first-zellij-plugin/tree/main)


