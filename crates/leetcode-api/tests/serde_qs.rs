use leetcode_api::leetcode::question::qs_detail::Question;
use miette::IntoDiagnostic;

#[test]
fn feature() -> miette::Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let json = serde_json::json!({
        "content": "\"<p>Given an array of integers <code>nums</code>&nbsp;and an integer <code>target</code>, return <em>indices of the two numbers such that they add up to <code>target</code></em>.</p>\\n\\n<p>You may assume that each input would have <strong><em>exactly</em> one solution</strong>, and you may not use the <em>same</em> element twice.</p>\\n\\n<p>You can return the answer in any order.</p>\\n\\n<p>&nbsp;</p>\\n<p><strong class=\\\"example\\\">Example 1:</strong></p>\\n\\n<pre>\\n<strong>Input:</strong> nums = [2,7,11,15], target = 9\\n<strong>Output:</strong> [0,1]\\n<strong>Explanation:</strong> Because nums[0] + nums[1] == 9, we return [0, 1].\\n</pre>\\n\\n<p><strong class=\\\"example\\\">Example 2:</strong></p>\\n\\n<pre>\\n<strong>Input:</strong> nums = [3,2,4], target = 6\\n<strong>Output:</strong> [1,2]\\n</pre>\\n\\n<p><strong class=\\\"example\\\">Example 3:</strong></p>\\n\\n<pre>\\n<strong>Input:</strong> nums = [3,3], target = 6\\n<strong>Output:</strong> [0,1]\\n</pre>\\n\\n<p>&nbsp;</p>\\n<p><strong>Constraints:</strong></p>\\n\\n<ul>\\n\\t<li><code>2 &lt;= nums.length &lt;= 10<sup>4</sup></code></li>\\n\\t<li><code>-10<sup>9</sup> &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>\\n\\t<li><code>-10<sup>9</sup> &lt;= target &lt;= 10<sup>9</sup></code></li>\\n\\t<li><strong>Only one valid answer exists.</strong></li>\\n</ul>\\n\\n<p>&nbsp;</p>\\n<strong>Follow-up:&nbsp;</strong>Can you come up with an algorithm that is less than&nbsp;<code>O(n<sup>2</sup>)&nbsp;</code>time complexity?\"",
        "stats": r#"{ "total_accepted": "4.6M", "total_submission": "8.8M", "total_accepted_raw": 4634785, "total_submission_raw": 8757109, "ac_rate": "52.9%" }"#,
        "sample_test_case": "[2,7,11,15]\n9",
        "example_testcases": "[2,7,11,15]\n9\n[3,2,4]\n6\n[3,3]\n6",
        "meta_data": r#"{ "name": "twoSum", "params": [ { "name": "nums", "type": "integer[]" }, { "name": "target", "type": "integer" } ], "return": { "type": "integer[]" } }"#,
        "translated_title": "\"两数之和\"",
        "translated_content": "\"<p>给定一个整数数组 <code>nums</code>&nbsp;和一个整数目标值 <code>target</code>，请你在该数组中找出 <strong>和为目标值 </strong><em><code>target</code></em>&nbsp; 的那&nbsp;<strong>两个</strong>&nbsp;整数，并返回它们的数组下标。</p>\\n\\n<p>你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。</p>\\n\\n<p>你可以按任意顺序返回答案。</p>\\n\\n<p>&nbsp;</p>\\n\\n<p><strong class=\\\"example\\\">示例 1：</strong></p>\\n\\n<pre>\\n<strong>输入：</strong>nums = [2,7,11,15], target = 9\\n<strong>输出：</strong>[0,1]\\n<strong>解释：</strong>因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。\\n</pre>\\n\\n<p><strong class=\\\"example\\\">示例 2：</strong></p>\\n\\n<pre>\\n<strong>输入：</strong>nums = [3,2,4], target = 6\\n<strong>输出：</strong>[1,2]\\n</pre>\\n\\n<p><strong class=\\\"example\\\">示例 3：</strong></p>\\n\\n<pre>\\n<strong>输入：</strong>nums = [3,3], target = 6\\n<strong>输出：</strong>[0,1]\\n</pre>\\n\\n<p>&nbsp;</p>\\n\\n<p><strong>提示：</strong></p>\\n\\n<ul>\\n\\t<li><code>2 &lt;= nums.length &lt;= 10<sup>4</sup></code></li>\\n\\t<li><code>-10<sup>9</sup> &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>\\n\\t<li><code>-10<sup>9</sup> &lt;= target &lt;= 10<sup>9</sup></code></li>\\n\\t<li><strong>只会存在一个有效答案</strong></li>\\n</ul>\\n\\n<p>&nbsp;</p>\\n\\n<p><strong>进阶：</strong>你可以想出一个时间复杂度小于 <code>O(n<sup>2</sup>)</code> 的算法吗？</p>\\n\"",
        "hints": [
            "A really brute force way would be to search for all possible pairs of numbers but that would be too slow. Again, it's best to try out brute force solutions for just for completeness. It is from these brute force solutions that you can come up with optimizations.",
            "So, if we fix one of the numbers, say <code>x</code>, we have to scan the entire array to find the next number <code>y</code> which is <code>value - x</code> where value is the input parameter. Can we change our array somehow so that this search becomes faster?",
            "The second train of thought is, without changing the array, can we use additional space somehow? Like maybe a hash map to speed up the search?"
        ],
        "mysql_schemas": [],
        "data_schemas": [],
        "question_id": "1",
        "question_title": "\"Two Sum\"",
        "is_paid_only": false,
        "code_snippets": [
            {
                "lang": "C++",
                "lang_slug": "cpp",
                "code": "class Solution {\npublic:\n    vector<int> twoSum(vector<int>& nums, int target) {\n        \n    }\n};"
            },
            {
                "lang": "Java",
                "lang_slug": "java",
                "code": "class Solution {\n    public int[] twoSum(int[] nums, int target) {\n\n    }\n}"
            },
            {
                "lang": "Python",
                "lang_slug": "python",
                "code": "class Solution(object):\n    def twoSum(self, nums, target):\n        \"\"\"\n        :type nums: List[int]\n        :type target: int\n        :rtype: List[int]\n        \"\"\""
            },
            {
                "lang": "Python3",
                "lang_slug": "python3",
                "code": "class Solution:\n    def twoSum(self, nums: List[int], target: int) -> List[int]:"
            },
            {
                "lang": "C",
                "lang_slug": "c",
                "code": "/**\n * Note: The returned array must be malloced, assume caller calls free().\n */\nint* twoSum(int* nums, int numsSize, int target, int* returnSize){\n\n}"
            },
            {
                "lang": "C#",
                "lang_slug": "csharp",
                "code": "public class Solution {\n    public int[] TwoSum(int[] nums, int target) {\n\n    }\n}"
            },
            {
                "lang": "JavaScript",
                "lang_slug": "javascript",
                "code": "/**\n * @param {number[]} nums\n * @param {number} target\n * @return {number[]}\n */\nvar twoSum = function(nums, target) {\n\n};"
            },
            {
                "lang": "Ruby",
                "lang_slug": "ruby",
                "code": "# @param {Integer[]} nums\n# @param {Integer} target\n# @return {Integer[]}\ndef two_sum(nums, target)\n\nend"
            },
            {
                "lang": "Swift",
                "lang_slug": "swift",
                "code": "class Solution {\n    func twoSum(_ nums: [Int], _ target: Int) -> [Int] {\n\n    }\n}"
            },
            {
                "lang": "Go",
                "lang_slug": "golang",
                "code": "func twoSum(nums []int, target int) []int {\n\n}"
            },
            {
                "lang": "Scala",
                "lang_slug": "scala",
                "code": "object Solution {\n    def twoSum(nums: Array[Int], target: Int): Array[Int] = {\n\n    }\n}"
            },
            {
                "lang": "Kotlin",
                "lang_slug": "kotlin",
                "code": "class Solution {\n    fun twoSum(nums: IntArray, target: Int): IntArray {\n\n    }\n}"
            },
            {
                "lang": "Rust",
                "lang_slug": "rust",
                "code": "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n\n    }\n}"
            },
            {
                "lang": "PHP",
                "lang_slug": "php",
                "code": "class Solution {\n\n    /**\n     * @param Integer[] $nums\n     * @param Integer $target\n     * @return Integer[]\n     */\n    function twoSum($nums, $target) {\n\n    }\n}"
            },
            {
                "lang": "TypeScript",
                "lang_slug": "typescript",
                "code": "function twoSum(nums: number[], target: number): number[] {\n\n};"
            },
            {
                "lang": "Racket",
                "lang_slug": "racket",
                "code": "(define/contract (two-sum nums target)\n  (-> (listof exact-integer?) exact-integer? (listof exact-integer?))\n\n  )"
            },
            {
                "lang": "Erlang",
                "lang_slug": "erlang",
                "code": "-spec two_sum(Nums :: [integer()], Target :: integer()) -> [integer()].\ntwo_sum(Nums, Target) ->\n  ."
            },
            {
                "lang": "Elixir",
                "lang_slug": "elixir",
                "code": "defmodule Solution do\n  @spec two_sum(nums :: [integer], target :: integer) :: [integer]\n  def two_sum(nums, target) do\n\n  end\nend"
            },
            {
                "lang": "Dart",
                "lang_slug": "dart",
                "code": "class Solution {\n  List<int> twoSum(List<int> nums, int target) {\n\n  }\n}"
            }
        ],
        "title": "Two Sum",
        "difficulty": "Easy",
        "topic_tags": [
            {
                "name": "Array",
                "slug": "array",
                "translated_name": "数组"
            },
            {
                "name": "Hash Table",
                "slug": "hash-table",
                "translated_name": "哈希表"
            }
        ]
    });
    // let mut detail = Question::parser_question(pb_data, pb.question_title_slug);

    let detail = serde_json::from_value::<Question>(json).into_diagnostic()?;
    // detail.qs_slug = Some(pb.question_title_slug);
    dbg!(&detail);
    let question_string = serde_json::to_string(&detail).unwrap_or_default();
    println!("{}", question_string);

    Ok(())
}
