# vim-zellij-navigator

Seamlessly integrate switching panes and tabs between Zellij and Neovim.

This is basically just rewritten [nvim-tmux-navigator](https://github.com/alexghergh/nvim-tmux-navigation/).
Just like the original, this works very simply:

```rust
if is_in_vim {
    send_keys("<C-l>");
} else {
    move_focus(Direction::Left);
}
```

## Setup

Download the wasm file from the [releases
page](https://github.com/PerchunPak/vim-zellij-navigator/releases). Save it to
your Zellij config path (e.g., `~/.config/zellij/plugins/vim-zellij-navigator.wasm`).
You will reference this path when defining the plugin in your Zellij config.

Then add this to your config:

```kdl
plugins {
    vim-zellij-navigator location="file:~/.config/zellij/plugins/vim-zellij-navigator.wasm" {
        // Exact matches on Vim binaries
        // This is how the plugin knows when you are inside Vim
        vim_commands "nvim|vim"
        // Print debug info to Zellij log?
        // Access with `tail -f /tmp/zellij-$(id -u)/zellij-log/zellij.log`
        print_to_log false
    }
    //...
}

load_plugins {
    vim-zellij-navigator
}

keybinds {
    shared_among "normal" "locked" "scroll" {
        // valid values: up, down, left, left-or-tab, right, right-or-tab
        bind "Ctrl h" { MessagePlugin "zellij-nvim" { payload "left-or-tab"; }; }
        bind "Ctrl l" { MessagePlugin "zellij-nvim" { payload "right-or-tab"; }; }
        bind "Ctrl j" { MessagePlugin "zellij-nvim" { payload "down"; }; }
        bind "Ctrl k" { MessagePlugin "zellij-nvim" { payload "up"; }; }

        unbind "Alt h" "Alt l" "Alt j" "Alt k"
        unbind "Alt left" "Alt right" "Alt down" "Alt up"
    }
}
```

This works perfectly with "unlock-first" preset described
[here](https://zellij.dev/documentation/keybinding-presets.html#the-unlock-first-non-colliding-preset).

## Troubleshooting

If you experience issues with the plugin, first try opening a fresh Zellij
session. If that doesn't work, clear the Zellij cache and restart Zellij
(`rm -rf ~/.cache/zellij` on Linux; `rm -rf ~/Library/Caches/org.Zellij-Contributors.Zellij`
on macOS)

[Zellij logs](https://zellij.dev/documentation/plugin-api-logging) are viewable
here on Linux:

```sh
tail -f /tmp/zellij-$(id -u)/zellij-log/zellij.log
```

On MacOS, you'll have to hunt for it in the directory `/var/folders`. This is
what I use to easily find and tail the Zellij log on MacOS:

```sh
find /var/folders -type f -name 'zellij.log' -exec tail -f {} \; 2>/dev/null
```

## Shoutouts

- [zellij-autolock](https://github.com/fresh2dev/zellij-autolock): this is actually a fork of autolock's codebase
- [hiasr/vim-zellij-navigator](https://github.com/hiasr/vim-zellij-navigator)
- [christoomey/vim-tmux-navigator](https://github.com/christoomey/vim-tmux-navigator)
- [dj95/zjstatus](https://github.com/dj95/zjstatus)
