# 在终端下刷力扣

<! 【[中文文档](./README-CN.md)】

<!--toc:start-->

-   [Brush leetcode under the terminal](#brush-leetcode-under-the-terminal)
    -   [Features](#features)
    -   [Install](#install)
    -   [Useage](#useage)
    -   [Videos](#videos)
    -   [Configuration](#configuration)
        -   [Important](#important)
        -   [Here are the explanations for each field](#here-are-the-explanations-for-each-field)
    -   [Tui Keymap](#tui-keymap)
    -   [Fuzzy Search](#fuzzy-search)
    <!--toc:end-->

## Features

-   Open the editor of your choice for editing.
-   Filter based on the category of the question.
-   Perform a fuzzy search.
-   Test and submit the code.
-   Modify the test case.
-   Automatically get cookies to eliminate the need for manual copying from
    the browser(support for a few specific browsers and platforms only,
    as adapting to various browsers and platforms can be complicated).

## Install

**Linux** Option dependencies(a implement `SecretService` service Application)：

-   `gnome-keyring`
-   `kwallet`
-   `KeePassXC`
-   `libsecret`

---

-   stable

```shell
rustup default nightly
cargo install lcode
```

## Usage

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

## Videos

<https://github.com/saying121/leetcode-cn-en-cli/assets/74663483/57a633e5-6bae-4816-a224-d7d61d2141af>

<https://github.com/saying121/leetcode-cn-en-cli/assets/74663483/7917a65c-b7a9-4305-b87f-5d2ddc8cb760>

![filter en](./pictures/filter_en.png)

## Configuration

[keymap](./KEYMAP.md)

The configuration located

-   Linux: `~/.config/leetcode-cn-en-cli/config.toml`
-   macos: `~/.config/leetcode-cn-en-cli/config.toml`
-   Windows: `|C:\Users\Alice\AppData\Roaming`

The code default located

-   Linux: `$HOME/.local/share/leetcode-cn-en-cli/`
-   macOS: `$HOME/Library/Application Support/leetcode-cn-en-cli`
-   Windows: `C:\Users\Alice\AppData\Local\leetcode-cn-en-cli`

default:
![default](./pictures/screen_shot_.png)

```toml
translate = false
column = 4
num_sublist = 10
page_size = 25
editor = ["vim"]
lang = "rust"
code_dir = "/home/user/.local/share/leetcode-cn-en-cli"

url_suffix = "com"
```

### Important

`~/.config/leetcode-cn-en-cli/cookies.toml`

```toml
csrf = ""
session = ""
```

**First, login leetcode in browser for generate cookies**

`browser` can fill in `chrome`, `edge`, `firefox`, `librewolf`.

Now support this browser, and just test in Linux.(firefox should support three systems)
If use the section，be careful not to clear cookies when closing the browser.

`[cookies]` section

-   If the two subfields are not empty,use the content filled by the user.
    And not use other method to get cookies。

    -   Fill in manually：

        Press <kbd>F12</kbd> on the browser's `leetcode.com/com` page,
        Find the **Cookie** field, copy the **`csrftoken`=\<content\>;**
        and **`LEETCODE_SESSION`=\<content\>;** sections inside it into the configuration.

-   If user filled `browser` , will try to use the browser to get cookies.

-   If neither of the above two items is filled in,
-   然后如果用户填写了 `browser` ，则会尝试所填写浏览器获取 cookies 。

-   以上两个都没有填写则会自动以 _firefox_ -> _edge_ -> _chrome_ -> _librewolf_ 的顺序尝试获取 cookies 。

### 各个字段的说明

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

```toml
[support_lang.rust]
start = "//start/"
end = "//end/"
inject_start = ""
inject_end = "struct Solution;\n\nfn main() {\n    println!(\"{:#?}\", Solution::function);\n}"
[support_lang.c]
...
```

会根据这些生成代码模板

可以写为多行,`"""..."""`或者`'''...'''`：

```toml
inject_end = """struct Solution;
fn main() {
    println!("{:#?}", Solution::function());
}"""
```

例如: 108

```rust
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// start /
// ...something
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(mut nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = nums.len();
        if len == 0 {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(nums[len / 2])));
        let mut right = nums.split_off(len / 2);
        right.remove(0);
        root.borrow_mut().left = Self::sorted_array_to_bst(nums);
        root.borrow_mut().right = Self::sorted_array_to_bst(right);

        Some(root)
    }
}
// end /

struct Solution;

fn main() {
    println!(
        "{:#?}",
        Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9])
    );
}
```

在提交至力扣时只会提交 `language.start` 和 `language.start` 之间的内容,
如果没有找到这两个部分就提交全部内容。

## 模糊搜索

模糊搜索 tui 和 cli 的实现是一样的， cli 的 paid only 是 true ，在 tui 也可以输入 `true`/`P.O.: tru` 来筛选
