# A toy project

-   【[中文文档](./README-CN.md)】

<!--toc:start-->
- [A toy project](#a-toy-project)
  - [Install](#install)
  - [Useage](#useage)
  - [视频](#视频)
  - [Configuration](#configuration)
    - [First](#first)
    - [Here are the explanations for each field](#here-are-the-explanations-for-each-field)
  - [Keymap](#keymap)
<!--toc:end-->

## Install

```shell
cargo install --git=https://github.com/saying121/leetcode-cn-en-cli.git --rev=c0094fb --force
```

## Useage

Generate configuration, manual modification of the configuration is also possible,
and it will be automatically generated at runtime.
Without -c, it will be generated in English.

```shell
lcode gencon -c
```

Synchronize basic data first.

```shell
lcode sync
```

View the documentation for assistance.

```shell
lcode -h
```

Begin selecting a question.

```shell
lcode fzy <edit>
```

## 视频

https://github.com/saying121/leetcode-cn-en-cli/assets/74663483/57a633e5-6bae-4816-a224-d7d61d2141af

https://github.com/saying121/leetcode-cn-en-cli/assets/74663483/7917a65c-b7a9-4305-b87f-5d2ddc8cb760

## Configuration

The configuration located

-   Linux: `~/.config/leetcode-cn-en-cli/config.toml`
-   macos: `~/.config/leetcode-cn-en-cli/config.toml`
-   Windows: `|C:\Users\Alice\AppData\Roaming`

```toml
translate = true
column = 4
num_sublist = 10
page_size = 25
editor = ["vim"]
lang = "rust"
code_dir = "/home/user/.local/share/leetcode-cn-en-cli"

[urls]
origin = "https://leetcode.com"
graphql = "https://leetcode.com/graphql"
all_problem_api = "https://leetcode.com/api/problems/$category"
submit = "https://leetcode.com/problems/$slug/submit/"
test = "https://leetcode.com/problems/$slug/interpret_solution/"
submissions = "https://leetcode.com/submissions/detail/$id/check/"
favorites = "https://leetcode.com/list/api/questions"

[support_lang]
langs = ["rust", "bash", "c", "cpp", "csharp", "golang", "java", "javascript", "kotlin", "mysql", "php", "python", "python3", "ruby", "scala", "swift", "typescript", "racket", "erlang", "elixir", "dart"]

[cookies]
csrf = ""
session = ""
```

### First

Press <kbd>F12</kbd> on the browser's `leetcode.com/com` page,
Find the cookie field, copy the **csrf** and **session** sections inside it into the configuration.

### Here are the explanations for each field

Fill in `false` or `true`, default is `false`.
If `true` is chosen, the translated content will be used to display the question details.

```toml
translate = true
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

Fill in your editor, it will attempt to retrieve it from the environment variables EDITOR and VISUAL,
otherwise it will default to vim.

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

```toml
code_dir = "/home/user/.local/share/leetcode-cn-en-cli"
```

## Keymap

|              key               |     global     |
| :----------------------------: | :------------: |
| <kbd>Shift-Tab/⬅/➡/Tab</kbd> | prev/next tab  |
|       <kbd>Ctrl-r</kbd>        | refresh screen |
|       <kbd>Ctrl-q</kbd>        |      exit      |

|       key        |      tab0/select      |
| :--------------: | :-------------------: |
|  <kbd>j/k</kbd>  |     down/up item      |
| <kbd>gg/G</kbd>  |      first/last       |
|   <kbd>o</kbd>   | open with your editor |
| <kbd>Enter</kbd> |      go to edit       |

|        key        |             tab1/edit              |
| :---------------: | :--------------------------------: |
|  <kbd>j/k</kbd>   |          scroll question           |
|  <kbd>gg/G</kbd>  |      question content top/end      |
| <kbd>ctrl-t</kbd> |          toggle submit menu          |
|   <kbd>S</kbd>    | Submit code(just show submit menu) |
|   <kbd>T</kbd>    |  Test code(just show submit menu)  |
