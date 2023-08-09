# 一个玩具项目

<!--toc:start-->
- [一个玩具项目](#一个玩具项目)
  - [安装](#安装)
  - [使用](#使用)
  - [配置](#配置)
    - [首先](#首先)
    - [各个字段的说明](#各个字段的说明)
<!--toc:end-->

## 安装

```shell
cargo install lcode
```

## 使用

生成配置，手动改配置也可以，在运行时会自动生成，
不带 `-c` 会以英文来生成

```shell
lcode gencon -c
```

先同步基本数据

```shell
lcode sync
```

查看帮助文档，开始选择题目

```shell
lcode -h
lcode fzy <edit>
```

https://github.com/saying121/leetcode-cn-en-cli/assets/74663483/62b8f4cc-73dc-49db-a6a1-4823a640a13a

## 配置

配置在 `~/.config/leetcode-cn-en-cli/config.toml`

```toml
tongue = "cn"
column = 4
num_sublist = 10
page_size = 25
editor = ["vim"]
lang = "rust"
code_dir = "/home/user/.local/share/leetcode-cn-en-cli"

[urls]
origin = "https://leetcode.cn"
graphql = "https://leetcode.cn/graphql"
all_problem_api = "https://leetcode.cn/api/problems/$category"
submit = "https://leetcode.cn/problems/$slug/submit/"
test = "https://leetcode.cn/problems/$slug/interpret_solution/"
submissions = "https://leetcode.cn/submissions/detail/$id/check/"
favorites = "https://leetcode.cn/list/api/questions"

[support_lang]
langs = ["rust", "bash", "c", "cpp", "csharp", "golang", "java", "javascript", "kotlin", "mysql", "php", "python", "python3", "ruby", "scala", "swift", "typescript", "racket", "erlang", "elixir", "dart"]

[cookies]
csrf = ""
session = ""
```

### 首先

从浏览器的 `leetcode.com/cn` 页面按下 <kbd>F12</kbd> ，
找到 **cookie** 字段，复制里面的 **csrf** 和 **session** 部分到配置里面。

### 各个字段的说明

填入 **cn** 或者 **en** ，默认 **en**

```toml
tongue = "cn"
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
