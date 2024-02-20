# LANGS

<!--toc:start-->
- [LANGS](#langs)
<!--toc:end-->

## LOCATION

`~/.config/leetcode-cn-en-cli/langs.toml`

## FIELD

```toml
[rust]
start = "//start/"
end = "//end/"
inject_start = ""
inject_end = "struct Solution;\n\nfn main() {\n    println!(\"{:#?}\", Solution::function());\n}"
[c]
```

will from this generate code template.

can write multi line,`"""..."""` or `'''...'''`：

```toml
inject_end = """struct Solution;
fn main() {
    println!("{:#?}", Solution::function());
}"""
```

Example: 108

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

When submit to leetcode, only content between `language.start`
and `language.start` will be uploaded.

If don't have this will uploaded all content.
