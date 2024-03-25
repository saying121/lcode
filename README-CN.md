# 终端写力扣

<!--toc:start-->
- [终端写力扣](#终端写力扣)
  - [功能](#功能)
  - [安装](#安装)
  - [使用](#使用)
  - [视频](#视频)
  - [配置](#配置)
    - [Cookies 重要部分](#cookies-重要部分)
    - [Keymap](#keymap)
    - [LANGS](#langs)
    - [CONFIG](#config)
    - [各个字段的说明](#各个字段的说明)
  - [模糊搜索](#模糊搜索)
  - [用户信息](#用户信息)
<!--toc:end-->

## 功能

- 调用给定的编辑器进行编辑。
- 根据题目类别过滤。
- 模糊搜索。
- 修改测试用例。
- 测试，提交代码。
- 自动获取 cookies 省去去浏览器复制的麻烦,
  详情:[decrypt-cookies](https://github.com/saying121/tidy-browser/tree/master/crates/decrypt-cookies#test-status)。

## 安装

依赖:

- `gcc`
- `libsecret` (Linux Optional)
- `libdbus` (Linux notify)
- [`mdcat`](https://github.com/swsnr/mdcat/) (渲染markdown)

---

• 使用最新的夜间工具链

```shell
rustup default nightly
cargo install --locked --force lcode
```

## 使用

生成配置，手动改配置也可以，在运行时会自动生成，
不带 `-c` 会以英文来生成

```shell
lcode gencon -c
```

先同步基本数据

```shell
lcode S
```

查看帮助文档，开始选择题目

```shell
lcode -h
lcode fzy <edit>
```

## 视频

https://github.com/saying121/leetcode-cn-en-cli/assets/74663483/62b8f4cc-73dc-49db-a6a1-4823a640a13a

https://github.com/saying121/leetcode-cn-en-cli/assets/74663483/7917a65c-b7a9-4305-b87f-5d2ddc8cb760

![filter cn](./pictures/filter_cn.png)

## 配置

### Cookies 重要部分

一般来说只需要填写 `~/.config/leetcode-cn-en-cli/config.toml`

```toml
browser = "edge" # `chrome`, `edge`, `firefox`, `librewolf` etc.
# 大小写无所谓, `eDgE` 也可以.
```

详情: [decrypt-cookies](https://github.com/saying121/tidy-browser/tree/master/crates/decrypt-cookies#test-status)

`~/.config/leetcode-cn-en-cli/cookies.toml`

```toml
csrf = ""
session = ""
```

> [!**首先在浏览器登陆 leetcode 来生成 cookies 。**]

`config.toml` 的 `browser` 字段可以填入 `chrome`, `edge`, `firefox`, `librewolf`。

目前只支持这几个浏览器，而且只在 Linux 系统测试过。(firefox 应该支持三个系统)
如果要使用这个选项，注意不要设置关闭浏览器时清空 cookies。

`[cookies]` 部分

- 如果两个子字段不为空则使用用户填写的内容。并不会使用其他方法获取 cookies。

  - 手动填写方法：

    从浏览器的 `leetcode.com/cn` 页面按下 <kbd>F12</kbd> ，
    找到 **Cookie** 字段，复制里面的 **`csrftoken`=\<$内容\>;** 和 **`LEETCODE_SESSION`=\<$内容\>;**
    复制 **$内容** 到配置里面。

- 然后如果用户填写了 `browser` ，则会尝试所填写浏览器获取 cookies 。

- 以上两个都没有填写则会自动以
  _firefox_ -> _librewolf_ -> _chrome_ -> _edge_ -> chromium
  -> brave -> Yandex ->  Vivaldi -> Opera -> OperaGX -> CocCoc
  的顺序尝试获取 cookies 。

### Keymap

[keymap](./KEYMAP.md)

### LANGS

[langs](./LANGS.md)

### CONFIG

配置位置

- Linux: `~/.config/leetcode-cn-en-cli/config.toml`
- macos: `~/.config/leetcode-cn-en-cli/config.toml`
- Windows: `C:\Users\Alice\AppData\Roaming\config.toml`

代码默认位置

- Linux: `$HOME/.local/share`
- macOS: `$HOME/Library/Application Support`
- Windows: `C:\Users\Alice\AppData\Local`

默认:
![default](./pictures/screen_shot_.png)

cn 用户建议更改两处: `url_suffix = "cn"`, `translate = true`.

```toml
translate = true
column = 4
num_sublist = 10
page_size = 25
editor = ["vim"]
lang = "rust"
code_dir = "/home/user/.local/share/leetcode-cn-en-cli"
browser = ""

url_suffix = "cn"
```

### 各个字段的说明

查看 [Cookies 重要部分](#cookies-重要部分) 部分.

```toml
browser = false
```

填入 `false` 或者 `true` ，默认 `false`，`true` 会使用翻译后的内容显示题目详情。

```toml
translate = true
```

---

获取 `submissionlist` 时显示几列

```toml
column = 4
```

---

显示最后多少条 `submissionlist` 信息

```toml
num_sublist = 10
```

---

交互选择题目时一次显示几道题目

```toml
page_size = 25
```

---

填写你的编辑器，会尝试从环境变量 `EDITOR` 和 `VISUAL` 获取，
否则为 `vim`

```toml
editor = ["vim"]
```

可以在后面添加参数

```toml
editor = ["vim", "--noplugin"]
```

---

设置你所选编程语言

```toml
lang = "rust"
```

---

设置代码和测试用例存储的位置

```toml
code_dir = "/home/user/.local/share/leetcode-cn-en-cli"
```

也可以这样写，以`~`开头

```toml
code_dir = "~/.local/share/leetcode-cn-en-cli"
```

---

填入 `com` 或者 `cn`，来设置网站后缀 `leetcode.com` 或者 `leetcode.cn`

```toml
url_suffix = "cn"
```

---

为了更好的写 rust。 这会添加一个 `Cargo.toml` 文件

```toml
cargo_integr = true
```

---

## 模糊搜索

模糊搜索 tui 和 cli 的实现是一样的， cli 的 paid only 是 true ，在 tui 也可以输入 `true`/`P.O.: tru` 来筛选

## 用户信息

你可以查看 tui 的 infos/tab3 界面来确认 cookies 是有效的.
