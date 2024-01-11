use std::fmt::Display;

use ratatui::{style::Stylize, text::Line};
use serde::{Deserialize, Serialize};

use crate::render::Render;

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct RunResult {
    #[serde(default)]
    pub elapsed_time: u32,
    #[serde(default)]
    pub finished:     bool,
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
    pub task_name:    String,

    #[serde(default)]
    pub status_code: i64,
    #[serde(default)]
    pub status_msg:  String,

    #[serde(default)]
    pub question_id:     String,
    #[serde(default)]
    pub std_output:      String,
    #[serde(default)]
    pub expected_output: String,
    #[serde(default)]
    pub last_testcase:   String,

    #[serde(default)]
    pub code_answer:          Vec<String>,
    // #[serde(default)]
    // pub code_output: String, // test:vec,submit:string, delete the field
    #[serde(default)]
    pub compare_result:       String,
    #[serde(default)]
    pub correct_answer:       bool,
    #[serde(default)]
    pub expected_code_answer: Vec<String>,
    #[serde(default)]
    pub expected_code_output: Vec<String>,

    #[serde(default)]
    pub pretty_lang: String,
    #[serde(default)]
    pub lang:        String,

    #[serde(default)]
    pub memory:            u64,
    #[serde(default)]
    pub status_memory:     String,
    #[serde(default)]
    pub memory_percentile: Option<f64>,

    #[serde(default)]
    pub status_runtime:     String,
    #[serde(default)]
    pub runtime_percentile: Option<f64>,
    #[serde(default)]
    pub run_success:        bool,

    #[serde(default)]
    pub state: String,

    #[serde(default)]
    pub std_output_list: Vec<String>,
    #[serde(default)]
    pub submission_id:   String,

    #[serde(default)]
    pub task_finish_time: u64,

    #[serde(default)]
    pub total_correct:   Option<u64>,
    #[serde(default)]
    pub total_testcases: Option<u64>,

    // runtime error
    #[serde(default)]
    pub full_runtime_error: String,
    #[serde(default)]
    pub runtime_error:      String,

    // compile error
    #[serde(default)]
    pub compile_error:      String,
    #[serde(default)]
    pub full_compile_error: String,
}

impl Render for RunResult {
    fn to_md_str(&self, _with_env: bool) -> String {
        self.to_string()
    }
    fn to_tui_vec(&self) -> Vec<Line> {
        let mut status_msg_id = vec![
            vec![
                "  # Status Code: ".into(),
                self.status_code
                    .to_string()
                    .bold()
                    .cyan(),
                ", Msg: ".into(),
                self.status_msg
                    .clone()
                    .bold()
                    .cyan(),
            ]
            .into(),
            vec![
                "  • Lang: ".into(),
                self.pretty_lang
                    .clone()
                    .bold()
                    .cyan(),
            ]
            .into(),
        ];
        if !self.question_id.is_empty() {
            status_msg_id.push(
                vec![
                    "  • Question ID: ".into(),
                    self.question_id
                        .clone()
                        .bold()
                        .cyan(),
                ]
                .into(),
            );
        }

        // make it meaning
        if self.full_runtime_error.is_empty() && self.full_compile_error.is_empty() {
            let total_correct_test_case = vec![
                vec![
                    "  • Total correct: ".into(),
                    self.total_correct
                        .unwrap_or_default()
                        .to_string()
                        .bold()
                        .cyan(),
                ]
                .into(),
                vec![
                    "  • Total Testcases: ".into(),
                    self.total_testcases
                        .unwrap_or_default()
                        .to_string()
                        .bold()
                        .cyan(),
                ]
                .into(),
            ];

            status_msg_id.extend(total_correct_test_case);
        }
        if !self.last_testcase.is_empty() {
            let last_case = vec![vec![
                "  • Last Testcases: ".into(),
                self.last_testcase
                    .clone()
                    .bold()
                    .cyan(),
            ]
            .into()];
            status_msg_id.extend(last_case);
        }
        if !self.status_memory.is_empty() {
            let mut mem_time = vec![vec![
                "  • Memory: ".into(),
                self.status_memory
                    .clone()
                    .bold()
                    .cyan(),
            ]
            .into()];
            if self.memory_percentile.is_some() {
                mem_time.push(
                    vec![
                        "  • Memory Low Than: ".into(),
                        self.memory_percentile
                            .unwrap_or_default()
                            .to_string()
                            .bold()
                            .cyan(),
                        "%".into(),
                    ]
                    .into(),
                );
            }
            mem_time.push(
                vec![
                    "  • Runtime: ".into(),
                    self.status_runtime
                        .clone()
                        .bold()
                        .cyan(),
                ]
                .into(),
            );
            if self.runtime_percentile.is_some() {
                mem_time.push(
                    vec![
                        "  • Fast Than: ".into(),
                        self.runtime_percentile
                            .unwrap_or_default()
                            .to_string()
                            .bold()
                            .cyan(),
                        "%".into(),
                    ]
                    .into(),
                );
            }

            status_msg_id.extend(mem_time);
        }
        if !self.full_compile_error.is_empty() {
            let full_c_err: Vec<Line> = self
                .full_compile_error
                .split('\n')
                .map(|v| v.to_owned().into())
                .collect();
            let mut compile_err = vec!["  • Compile Error:".into()];
            compile_err.extend(full_c_err);

            status_msg_id.extend(compile_err);
        }
        if !self.full_runtime_error.is_empty() {
            let full_r_err: Vec<Line> = self
                .full_runtime_error
                .split('\n')
                .map(|v| v.to_owned().into())
                .collect();
            let mut runtime_err = vec!["  • Runtime Error:".into()];
            runtime_err.extend(full_r_err);

            status_msg_id.extend(runtime_err);
        }
        if !self.code_answer.is_empty() {
            let y_ans1 = self
                .code_answer
                .iter()
                .map(|v| format!("    • {v}").into())
                .collect::<Vec<Line>>();
            let mut your_ans = vec!["  • Your Answer:".into()];
            your_ans.extend(y_ans1);

            status_msg_id.extend(your_ans);
        }
        if !self
            .expected_code_answer
            .is_empty()
        {
            let c_ans1 = self
                .expected_code_answer
                .iter()
                .map(|v| format!("    • {}", v).into())
                .collect::<Vec<Line>>();
            let mut correct_ans = vec!["  • Correct Answer:".into()];
            correct_ans.extend(c_ans1);

            status_msg_id.extend(correct_ans);
        }
        let std_output = self
            .std_output_list
            .iter()
            .filter(|v| !v.is_empty())
            .map(|v| format!("    • {v}").into())
            .collect::<Vec<Line>>();
        if !std_output.is_empty() {
            let mut stdout_ans = vec!["  • Std Output:".into()];
            stdout_ans.extend(std_output);

            status_msg_id.extend(stdout_ans);
        }

        status_msg_id
    }
}

impl Display for RunResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut status_id_lang = format!(
            "\
            # Status Code: {scode}, Msg: {msg}\n* Lang: {lang}\n",
            scode = self.status_code,
            msg = self.status_msg,
            lang = self.pretty_lang,
        );
        if self.full_runtime_error.is_empty() && self.full_compile_error.is_empty() {
            let total_test_case = format!(
                "\
                * Total correct: {}\n* Total Testcases: {}\n",
                self.total_correct
                    .unwrap_or_default(),
                self.total_testcases
                    .unwrap_or_default(),
            );
            status_id_lang.push_str(&total_test_case);
        }
        if !self.last_testcase.is_empty() {
            let last_testcase = format!(
                "\
                * Last Testcases {}\n",
                self.last_testcase
            );
            status_id_lang.push_str(&last_testcase);
        }
        if !self.status_runtime.is_empty() {
            let mut run_time = format!(
                "\
            * Runtime: {}\n",
                self.status_runtime,
            );
            if self.runtime_percentile.is_some() {
                run_time.push_str(&format!(
                    "\
                * Fast Than: {}%\n",
                    self.runtime_percentile
                        .unwrap_or_default()
                ));
            }
            status_id_lang.push_str(&run_time);
        }
        if !self.status_memory.is_empty() {
            let mut run_memory = format!(
                "\
                * Memory: {}\n",
                self.status_memory,
            );
            if self.memory_percentile.is_some() {
                run_memory.push_str(&format!(
                    "* Memory Low Than: {}%\n",
                    self.memory_percentile
                        .unwrap_or_default()
                ));
            }

            status_id_lang.push_str(&run_memory);
        }
        if !self.full_compile_error.is_empty() {
            let compile_error = format!(
                "\
                * Compile Error:\n```\n{}\n```\n",
                self.full_compile_error
            );
            status_id_lang.push_str(&compile_error);
        }
        if !self.full_runtime_error.is_empty() {
            let runtime_err = format!(
                "\
                * Runtime Error:\n```\n{}\n```\n",
                self.full_runtime_error
            );
            status_id_lang.push_str(&runtime_err);
        }
        if !self.code_answer.is_empty() {
            let your_answer = format!(
                "\
                * Your Answer: \n{}\n",
                self.code_answer
                    .iter()
                    .map(|v| format!("    * {}", v))
                    .collect::<Vec<String>>()
                    .join("\n"),
            );

            status_id_lang.push_str(&your_answer);
        }
        if !self
            .expected_code_answer
            .is_empty()
        {
            let corr_answer = format!(
                "\
                * Correct Answer: \n{}\n",
                self.expected_code_answer
                    .iter()
                    .map(|v| format!("    * {}", v))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
            status_id_lang.push_str(&corr_answer);
        }
        let out_put = self.std_output_list.join("\n");
        if !out_put
            .trim_matches('\n')
            .is_empty()
        {
            let head = format!(
                "\
                * Std Output:\n{}\n",
                out_put
            );
            status_id_lang.push_str(&head);
        }

        status_id_lang.fmt(f)
    }
}
