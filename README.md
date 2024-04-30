# Brush leetcode under the terminal

- 【[中文文档](./README-CN.md)】

<!--toc:start-->
- [Brush leetcode under the terminal](#brush-leetcode-under-the-terminal)
  - [Features](#features)
  - [Install](#install)
    - [You can use any of the following methods to install](#you-can-use-any-of-the-following-methods-to-install)
    - [Update lcode](#update-lcode)
  - [Usage](#usage)
    - [Shell completion.](#shell-completion)
  - [Videos](#videos)
  - [Configuration](#️configuration)
    - [Cookies (Important)](#cookies-important)
    - [Keymap](#keymap)
    - [LANGS](#langs)
    - [CONFIG](#config)
    - [Here are the explanations for each field](#here-are-the-explanations-for-each-field)
  - [The User Infos](#-the-user-infos)
  - [Todo](#todo)
<!--toc:end-->

## 🪶Features

- Open the editor of your choice for editing.
- Filter based on the category of the question.
- Fuzzy search.
- Test and submit the code.
- Modify the test case.
- Automatically get cookies to eliminate the need for manual copying from
  the browser, power by [decrypt-cookies](https://github.com/saying121/tidy-browser/tree/master/crates/decrypt-cookies#test-status).

## 💄Install

dependencies:

- `libsecret` (Linux Optional)
- `libdbus` (Linux notify)
- [`mdcat`](https://github.com/swsnr/mdcat/) (render markdown)

build-dependencies:

- `gcc`
- `pkg-config` (when without cross feature)
- `libdbus-1-dev` (when without cross feature)

### You can use any of the following methods to install

> [!NOTE]
>
> add `~/.cargo/bin` to your `$PATH`

- Install binaries directly using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```bash
cargo binstall lcode
```

- Download from [release](https://github.com/saying121/lcode/releases)

In reality, [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)
is used to download binaries from
[release](https://github.com/saying121/lcode/releases)

- Build by yourself

```shell
rustup default nightly
cargo install --locked --force lcode
# or
# cargo install --features cross --locked --force lcode
```

### Update lcode

use [cargo-update](https://github.com/nabijaczleweli/cargo-update)

```bash
cargo install-update --all
```

## 🔧Usage

Generate configuration, manual modification of the configuration is also possible,
and it will be automatically generated at runtime.
Without -c, it will be generated in English.

```shell
lcode gencon -c
```

Synchronize basic data first.

```shell
lcode S
```

View the documentation for assistance.

```shell
lcode -h
```

Begin selecting a question.

```shell
lcode fzy <edit>
```

### Shell completion

```bash
# zsh
echo 'eval $(lcode --generate zsh)' >>~/.zshrc
# bash
echo 'eval $(lcode --generate bash)' >>~/.bashrc
# ...
```

- Use [zi](https://github.com/z-shell/zi)

```zsh
zi ice lucid wait as'completion' blockf has'lcode'
zi snippet https://github.com/saying121/lcode/blob/main/completions/_lcode
```

## 📼Videos

<https://github.com/saying121/lcode/assets/74663483/57a633e5-6bae-4816-a224-d7d61d2141af>

<https://github.com/saying121/lcode/assets/74663483/9ad6ad58-b401-42f6-b8dc-359f78a37729>

## ⚙️Configuration

The configuration located

- Linux: `~/.config/lcode/`
- macos: `~/.config/lcode/`
- Windows: `C:\Users\user\AppData\Roaming\lcode`

The code default located

- Linux: `~/.local/share/lcode/`
- macOS: `~/Library/Application Support/lcode`
- Windows: `C:\Users\user\AppData\Local\lcode`

code layout:
![default](./pictures/screen_shot_.png)

The cache located

- Linux: `~/.local/share/lcode/`
- macOS: `~/Library/Caches/lcode`
- Windows: `C:\Users\user\AppData\Local\lcode`

### Cookies (Important)

> [**First, login leetcode in browser for generate cookies**]

General you just need filled browser at `~/.config/lcode/config.toml`.

When use the section，be careful not to clear cookies when closing the browser.

```toml
browser = "edge" # `chrome`, `edge`, `firefox`, `librewolf` etc.
# Not casesensitive, `eDgE` also ok.
```

The detail: [decrypt-cookies](https://github.com/saying121/tidy-browser/tree/master/crates/decrypt-cookies#test-status)

---

`~/.config/lcode/cookies.toml`

```toml
csrf = ""
session = ""
```

`[cookies]` section

- If the two fields are not empty, use the content filled by the user.
  And not use other method to get cookies。

  - Fill in manually：

    Press <kbd>F12</kbd> on the browser's `leetcode.com/com` page, click network.
    Find the **Cookie** field, copy the **`csrftoken`=\<$content\>;**
    and **`LEETCODE_SESSION`=\<$content\>;** copy the **$content** into the configuration.

- If filled `browser` , will try to use the browser to get cookies.

- If neither of the above two items is filled in,
  and then use this order
  _Firefox_ -> _Librewolf_ -> _Chrome_ -> _Edge_ -> _Chromium_
  -> _Brave_ -> _Yandex_ -> _Vivaldi_ -> _Opera_ -> _OperaGX_ -> _CocCoc_
  try to get cookies.

### Keymap

[keymap](./KEYMAP.md)

### LANGS

[langs](./LANGS.md)

### CONFIG

```toml
translate = false
column = 4
num_sublist = 10
page_size = 25
editor = ["vim"]
lang = "rust"
code_dir = "/home/user/.local/share/lcode"
browser = ""

url_suffix = "com"
```

### Here are the explanations for each field

Checkout the [Cookies (Important)](#cookies-important) section above.

```toml
browser = ""
```

---

Boolean, default is `false`. Show translated content or not.

```toml
translate = false
```

---

When retrieving the **submissionlist**, how many columns should be displayed.

```toml
column = 4
```

---

How many recent entries of the submissionlist information should be displayed.

```toml
num_sublist = 10
```

---

How many questions should be displayed at once when interactively selecting a question.

```toml
page_size = 25
```

---

Fill in your editor, it will attempt to retrieve it from
the environment variables `$EDITOR` and `$VISUAL`,
otherwise it will default to `vim`.

Specifically, when the editor is one of the ones below,
will vert split question and code.

- vim
- nvim
- helix

```toml
editor = ["vim"]
```

You can add additional parameters at the end.

```toml
editor = ["vim", "--noplugin"]
```

---

Set your selected programming language.

```toml
lang = "rust"
```

---

Set the location for storing code and test cases.
You can also starting with `~`

```toml
code_dir = "/home/user/.local/share/lcode"
# or
code_dir = "~/.local/share/lcode"
```

---

Fill in `com` or `cn`, for set `leetcode.com` or `leetcode.cn`.

```toml
url_suffix = "com"
```

---

For better rust coding. It will add a `Cargo.toml` file

```toml
cargo_integr = true
```

## 👣 The User Infos

You can checkout the infos/tab3 in tui for ensure cookies is valid.

## Todo

- cache cookies
