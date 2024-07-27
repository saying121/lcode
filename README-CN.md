# 终端写力扣

<!--toc:start-->
- [终端写力扣](#终端写力扣)
  - [功能](#功能)
  - [安装](#安装)
    - [可以使用以下任意一种方法安装](#可以使用以下任意一种方法安装)
    - [更新 lcode](#更新-lcode)
  - [使用](#使用)
  - [视频](#视频)
  - [配置](#配置)
    - [Cookies 重要部分](#cookies-重要部分)
    - [Keymap](#keymap)
    - [LANGS](#langs)
    - [CONFIG](#config)
    - [各个字段的说明](#各个字段的说明)
  - [用户信息](#用户信息)
<!--toc:end-->

> [!WARNING]
>
> 此文档有可能更新不及时，以英文文档为准。

## 功能

- 调用给定的编辑器进行编辑。
- 根据题目类别过滤。
- 模糊搜索。
- 修改测试用例。
- 测试，提交代码。
- 自动获取 cookies 省去去浏览器复制的麻烦,
  详情：[decrypt-cookies](https://github.com/saying121/tidy-browser/tree/master/crates/decrypt-cookies#test-status)。

## 安装

依赖:

- `sqlite`
- `libsecret` (Linux Optional)
- `libdbus` (Linux 通知)
- [`mdcat`](https://github.com/swsnr/mdcat/) (渲染 markdown)

构建依赖:

- `gcc`
- `pkg-config`
- `libdbus-1-dev`

### 可以使用以下任意一种方法安装

> [!NOTE]
>
> 添加 `~/.cargo/bin` 到 `$PATH`

- 通过 [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) 直接安装二进制。

```bash
cargo binstall lcode
```

- Archlinux 可以使用 AUR 助手 安装

AUR 里面打包了 bash, zsh, fish 的补全脚本。

```bash
yay -S lcode-bin
# or
paru -S lcode-bin
```

- 去 [release](https://github.com/saying121/lcode/releases) 下载

[release](https://github.com/saying121/lcode/releases)

- 自己编译

```shell
rustup default nightly
cargo install --locked --force lcode
```

### 更新 lcode

使用 [cargo-update](https://github.com/nabijaczleweli/cargo-update)

```bash
cargo install-update --all
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

Shell 补全.

```bash
# zsh
echo 'eval $(lcode --generate zsh)' >>~/.zshrc
# bash
echo 'eval $(lcode --generate bash)' >>~/.bashrc
# ...
```

## 视频

<https://github.com/saying121/lcode/assets/74663483/62b8f4cc-73dc-49db-a6a1-4823a640a13a>

<https://github.com/saying121/lcode/assets/74663483/9ad6ad58-b401-42f6-b8dc-359f78a37729>

## 配置

配置位置

- Linux: `~/.config/lcode/`
- macos: `~/.config/lcode/`
- Windows: `C:\Users\Alice\AppData\Roaming\lcode`

代码默认位置

- Linux: `~/.local/share/lcode`
- macOS: `~/Library/Application Support/lcode`
- Windows: `C:\Users\Alice\AppData\Local\lcode`

布局:

```txt
1_two-sum/
├── 1.cpp*
├── 1.rs*
├── 1_detail_cn.md*
├── 1_detail_en.md*
└── 1_test_case.txt*
```

缓存位置

- Linux: `~/.local/share/lcode/`
- macOS: `~/Library/Caches/lcode`
- Windows: `C:\Users\user\AppData\Local\lcode`

### Cookies 重要部分

一般来说只需要填写 `~/.config/lcode/config.toml`
要使用这个选项，注意不要设置关闭浏览器时清空 cookies。

```toml
browser = "edge" # `chrome`, `edge`, `firefox`, `librewolf` etc.
# 大小写无所谓, `eDgE` 也可以.
```

详情: [decrypt-cookies](https://github.com/saying121/tidy-browser/tree/master/crates/decrypt-cookies#test-status)

---

`~/.config/lcode/cookies.toml`

```toml
csrf = ""
session = ""
```

> [**首先在浏览器登陆 leetcode 来生成 cookies 。**]

`[cookies]` 部分

- 如果两个子字段**不为空**则使用用户填写的内容。并不会使用其他方法获取 cookies。

  - 手动填写方法：

    从浏览器的 `leetcode.com/cn` 页面按下 <kbd>F12</kbd>，点击 network/网络，
    找到 **Cookie** 字段，复制里面的 **`csrftoken`=\<$内容\>;** 和 **`LEETCODE_SESSION`=\<$内容\>;**
    复制 **$内容** 到配置里面。

- 如果填写了 `browser` ，则会尝试所填写浏览器获取 cookies 。

- 以上两个都没有填写则会自动以
  _Firefox_ -> _Librewolf_ -> _Chrome_ -> _Edge_ -> _Chromium_
  -> _Brave_ -> _Yandex_ -> _Vivaldi_ -> _Opera_ -> _OperaGX_ -> _CocCoc_
  的顺序尝试获取 cookies 。

### Keymap

[keymap](./KEYMAP.md)

### LANGS

[langs](./LANGS.md)

### CONFIG

运行 `lcode gencon -c` 生成配置，`lcode C` 编辑。

cn 用户建议更改两处: `url_suffix = "cn"`, `translate = true`.

```toml
# 是否显示翻译后的内容
translate = true

# 填入 `com` 或者 `cn`，来设置网站后缀 `leetcode.com` 或者 `leetcode.cn`
url_suffix = "cn"

# 获取 `submissionlist` 时显示几列
column = 4

# 显示最后多少条 `submissionlist` 信息
num_sublist = 10

# 交互选择题目时一次显示几道题目
page_size = 25

# 填写你的编辑器，会尝试从环境变量 `EDITOR` 和 `VISUAL` 获取，
# 否则为 `vim`
#
# 魔法，使用这些编辑器或垂直分割问题和编辑器。
#
# - vim
# - nvim
# - helix
#
#可以在后面添加参数
#例如 `editor = ["vim", "--noplugin"]`
editor = ["vim"]

# 设置你所选编程语言
lang = "rust"

# 设置代码和测试用例存储的位置
# 也可以这样写，以`~`开头
# 例如： `code_dir = "~/.local/share/lcode"`
code_dir = "/home/user/.local/share/lcode"

# 查看 [Cookies 重要部分](#cookies-重要部分) 部分.
browser = ""

# 为了更好的写 rust。 这会添加一个 `Cargo.toml` 文件
cargo_integr = true

# 是否用 fronted id 创建代码目录
dir_with_frontend_id = true
```

## 用户信息

你可以查看 tui 的 info/tab3 界面来确认 cookies 是有效的.
