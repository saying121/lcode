use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::render::Render;

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct RunResult {
    #[serde(default)]
    pub elapsed_time: u32,
    #[serde(default)]
    pub finished: bool,
    // pub expected_elapsed_time: u32,
    // pub expected_lang: String,
    // pub expected_memory: u128,
    // pub expected_run_success: bool,
    // pub expected_status_code: i32,
    // pub expected_status_runtime: String,
    // pub expected_std_output_list: Vec<String>,
    // pub expected_task_finish_time: u128,
    // pub expected_task_name: String,
    // pub fast_submit: bool,
    #[serde(default)]
    pub task_name: String,

    #[serde(default)]
    pub status_code: i64,
    #[serde(default)]
    pub status_msg: String,

    #[serde(default)]
    pub question_id: String,
    #[serde(default)]
    pub std_output: String,
    #[serde(default)]
    pub expected_output: String,
    #[serde(default)]
    pub last_testcase: String,

    #[serde(default)]
    pub code_answer: Vec<String>,
    // #[serde(default)]
    // pub code_output: String, // test:vec,submit:string, delete the field
    #[serde(default)]
    pub compare_result: String,
    #[serde(default)]
    pub correct_answer: bool,
    #[serde(default)]
    pub expected_code_answer: Vec<String>,
    #[serde(default)]
    pub expected_code_output: Vec<String>,

    #[serde(default)]
    pub pretty_lang: String,
    #[serde(default)]
    pub lang: String,

    #[serde(default)]
    pub memory: u64,
    #[serde(default)]
    pub status_memory: String,
    #[serde(default)]
    pub memory_percentile: Option<f64>,

    #[serde(default)]
    pub status_runtime: String,
    #[serde(default)]
    pub runtime_percentile: Option<f64>,
    #[serde(default)]
    pub run_success: bool,

    #[serde(default)]
    pub state: String,

    #[serde(default)]
    pub std_output_list: Vec<String>,
    #[serde(default)]
    pub submission_id: String,

    #[serde(default)]
    pub task_finish_time: u64,

    #[serde(default)]
    pub total_correct: Option<u64>,
    #[serde(default)]
    pub total_testcases: Option<u64>,

    // runtime error
    #[serde(default)]
    pub full_runtime_error: String,
    #[serde(default)]
    pub runtime_error: String,

    // compile error
    #[serde(default)]
    pub compile_error: String,
    #[serde(default)]
    pub full_compile_error: String,
}

impl Render for RunResult {
    fn to_tui_vec(&self) -> Vec<String> {
        let mut head = vec![
            format!(
                "# Status Code: {}, Msg: {}",
                self.status_code, self.status_msg
            ),
            format!("• Lang: {} ", self.pretty_lang),
        ];
        let test_case = vec![
            format!(
                "• Total correct {} ",
                self.total_correct
                    .unwrap_or_default()
            ),
            format!(
                "• Total Testcases {} ",
                self.total_testcases
                    .unwrap_or_default()
            ),
            format! {"• Last Testcases {}",self.last_testcase},
        ];
        let time = vec![
            format!("• Memory: {} ", self.status_memory),
            format!(
                "• Memory Exceed: {} ",
                self.memory_percentile
                    .unwrap_or_default()
            ),
            format!("• Runtime: {} ", self.status_runtime),
            format!(
                "• Fast Than: {} ",
                self.runtime_percentile
                    .unwrap_or_default()
            ),
        ];
        let full_r_err: Vec<String> = self
            .full_runtime_error
            .split("\n")
            .map(|v| v.to_string())
            .collect();
        let mut r_err = vec![format!("• Runtime Error:")];
        r_err.extend(full_r_err);
        let full_c_err: Vec<String> = self
            .full_compile_error
            .split("\n")
            .map(|v| v.to_string())
            .collect();
        let mut c_err = vec![format!("• Compile Error:")];
        c_err.extend(full_c_err);
        let y_ans1 = self
            .code_answer
            .iter()
            .map(|v| format!("    * {}", v))
            .collect::<Vec<String>>();
        let mut y_ans = vec![format!("• Your Answer:")];
        y_ans.extend(y_ans1);
        let c_ans1 = self
            .expected_code_answer
            .iter()
            .map(|v| format!("    * {}", v))
            .collect::<Vec<String>>();
        let mut c_ans = vec![format!("• Correct Answer:",)];
        c_ans.extend(c_ans1);

        match self.status_code {
            10 => {
                head.extend(test_case);
                head.extend(time);
                head.extend(y_ans);
                head.extend(c_ans);
            }
            11 => {
                head.extend(test_case);
                head.extend(time);
                head.extend(y_ans);
                head.extend(c_ans);
            }
            12 => {
                head.extend(test_case);
                head.extend(time);
                head.extend(y_ans);
                head.extend(c_ans);
            }
            15 => {
                head.extend(r_err);
            }
            20 => {
                head.extend(c_err);
            }
            _ => {
                head.extend(test_case);
                head.extend(time);
                head.extend(r_err);
                head.extend(c_err);
                head.extend(y_ans);
                head.extend(c_ans);
            }
        };

        head
    }
}

impl Display for RunResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status_code {
            // pass
            10 => format!(
                "# Status Code: {scode}, Msg: {msg} \n\
                * Lang: {lang} \n\
                * Total correct {tc} \n\
                * Total Testcases {tt} \n\
                * Memory: {mem} \n\
                * Memory Exceed: {mem} \n\
                * Runtime: {tim} \n\
                * Fast Than: {r_per} \n\
                * Your Answer: \n{ans} \n\
                * Correct Answer: \n{c_ans} ",
                scode = self.status_code,
                msg = self.status_msg,
                r_per = self
                    .runtime_percentile
                    .unwrap_or_default(),
                lang = self.pretty_lang,
                tc = self
                    .total_correct
                    .unwrap_or_default(),
                tt = self
                    .total_testcases
                    .unwrap_or_default(),
                mem = self.status_memory,
                tim = self.status_runtime,
                ans = self
                    .code_answer
                    .iter()
                    .map(|v| format!("    * {}", v))
                    .collect::<Vec<String>>()
                    .join("\n"),
                c_ans = self
                    .expected_code_answer
                    .iter()
                    .map(|v| format!("    * {}", v))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
            .fmt(f),
            // failed
            11 => format!(
                "# Status Code: {scode}, Msg: {msg} \n\
                * Lang: {lang} \n\
                * Total correct {tc} \n\
                * Total Testcases {tt} \n\
                * Last Testcases {ltc} \n\
                * Memory: {mem} \n\
                * Runtime: {tim} \n\
                * Your Answer: \n{ans} \n\
                * Correct Answer: \n{c_ans} ",
                scode = self.status_code,
                msg = self.status_msg,
                ltc = self.last_testcase,
                lang = self.pretty_lang,
                tc = self
                    .total_correct
                    .unwrap_or_default(),
                tt = self
                    .total_testcases
                    .unwrap_or_default(),
                mem = self.status_memory,
                tim = self.status_runtime,
                ans = self
                    .code_answer
                    .iter()
                    .map(|v| format!("    * {}", v))
                    .collect::<Vec<String>>()
                    .join("\n"),
                c_ans = self
                    .expected_code_answer
                    .iter()
                    .map(|v| format!("    * {}", v))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
            .fmt(f),
            // Memory Exceeded
            12 => format!(
                "# Status Code: {scode}, Msg: {msg} \n\
                * Lang: {lang} \n\
                * Total correct {tc} \n\
                * Total Testcases {tt} \n\
                * Memory: {mem} \n\
                * Runtime: {tim} \n\
                * Runtime Error:\n\
                ```\n\
                {rerr}\n\
                ```\n\
                \n\
                * Your Answer: \n{ans} \n\
                * Correct Answer: \n{c_ans} ",
                scode = self.status_code,
                msg = self.status_msg,
                rerr = self.full_runtime_error,
                lang = self.pretty_lang,
                tc = self
                    .total_correct
                    .unwrap_or_default(),
                tt = self
                    .total_testcases
                    .unwrap_or_default(),
                mem = self.status_memory,
                tim = self.status_runtime,
                ans = self
                    .code_answer
                    .iter()
                    .map(|v| format!("    * {}", v))
                    .collect::<Vec<String>>()
                    .join("\n"),
                c_ans = self
                    .expected_code_answer
                    .iter()
                    .map(|v| format!("    * {}", v))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
            .fmt(f),
            // Runtime error
            15 => format!(
                "# Status Code: {scode}, Msg: {msg} \n\
                * Lang: {lang} \n\
                * Runtime: {tim} \n\
                * Runtime Error:\n\
                ```\n\
                {rerr}\n\
                ```\n\
                \n",
                scode = self.status_code,
                msg = self.status_msg,
                rerr = self.full_runtime_error,
                lang = self.pretty_lang,
                tim = self.status_runtime,
            )
            .fmt(f),
            // Compile Error
            20 => format!(
                "# Status Code: {scode}, Msg: {msg} \n\
                * Lang: {lang} \n\
                * Compile Error:\n\
                ```\n\
                {cerr}\n\
                ```\n\
                \n\
                ",
                scode = self.status_code,
                msg = self.status_msg,
                cerr = self.full_compile_error,
                lang = self.pretty_lang,
            )
            .fmt(f),
            _ => format!(
                "# Status Code: {scode}, Msg: {msg} \n\
                * Lang: {lang} \n\
                * Total correct {tc} \n\
                * Total Testcases {tt} \n\
                * Last Testcases {ltc} \n\
                * Memory: {mem} \n\
                * Runtime: {tim} \n\
                * Runtime Error:\n\
                ```\n\
                {rerr}\n\
                ```\n\
                \n\
                * Compile Error:\n\
                ```\n\
                {cerr}\n\
                ```\n\
                \n\
                * Your Answer: \n{ans} \n\
                * Correct Answer: \n{c_ans} ",
                ltc = self.last_testcase,
                msg = self.status_msg,
                scode = self.status_code,
                rerr = self.full_runtime_error,
                cerr = self.full_compile_error,
                lang = self.pretty_lang,
                tc = self
                    .total_correct
                    .unwrap_or_default(),
                tt = self
                    .total_testcases
                    .unwrap_or_default(),
                mem = self.status_memory,
                tim = self.status_runtime,
                ans = self
                    .code_answer
                    .iter()
                    .map(|v| format!("    * {}", v))
                    .collect::<Vec<String>>()
                    .join("\n"),
                c_ans = self
                    .expected_code_answer
                    .iter()
                    .map(|v| format!("    * {}", v))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
            .fmt(f),
        }
    }
}
