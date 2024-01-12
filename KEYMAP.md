# keymap

<!--toc:start-->
- [keymap](#keymap)
  - [default Keymap](#default-keymap)
  - [custom keymap](#custom-keymap)
<!--toc:end-->

## default Keymap

|      key      |     global     |
| :-----------: | :------------: |
| Shift-Tab/Tab | prev/next tab  |
|    Ctrl-l     | refresh screen |
|    Ctrl-q     |      exit      |

|  key  |        tab0/select        |
| :---: | :-----------------------: |
|  j/k  |          down/up          |
| gg/G  |        first/last         |
|   o   |   open with your editor   |
| Enter |          trigger          |
|   S   | sync question information |

|  key   |             tab1/edit              |
| :----: | :--------------------------------: |
|  j/k   |          scroll question           |
|  gg/G  |      question content top/end      |
| ctrl-p |         toggle submit menu         |
| ctrl-s |        toggle submit result        |
| ctrl-t |         toggle test result         |
|   S    | Submit code(just show submit menu) |
|   T    |  Test code(just show submit menu)  |

Please check the Tui interface for specific keymap information.

## custom keymap

vim style keymap, upper camel case or lowcase

| key                | display                                                      |
| ------------------ | ------------------------------------------------------------ |
| backspace          | `<BS>/<Bs>`                                                  |
| space              | `<Space>/<space>`                                            |
| with Ctrl          | `<C-some_key>`                                               |
| with Shift         | `<S-some_key>`                                               |
| with Alt           | `<M-some_key>/<A-some_key>`                                  |
| left/right/up/down | `<left>/<right>/<up>/<down>` or `<Left>/<Right>/<Up>/<Down>` |
| a-z                | `a-z`                                                        |

You can write `keymap.toml` file like this

```toml
keymap = [
    { keys = "GG", action = "bottom" },
    { keys = "gg", action = "top" },
    { keys = "<S-l>", action = "top" },
    { keys = "<S-g>", action = "top" },
    { keys = "<Tab>", action = "next_tab" },
    { keys = "<S-Tab>a", action = "prev_tab" },
    { keys = "<S-Tab>b", action = "prev_tab" },
]
```

The same action item will keep one, and `"L"` is the same as `"<S-L>"` and `<S-l>`.
