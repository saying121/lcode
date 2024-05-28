#[cfg(feature = "ratatui")]
use ratatui::{style::Stylize, text::Line};

use super::Render;
use crate::leetcode::resps::run_res::RunResult;

impl RunResult {
    pub fn start_tui_text(&self) -> Vec<Line> {
        let total_testcases = self.total_testcases();
        let total_correct = self.total_correct();
        let line1 = "  # Status Code: ";
        let line2 = self
            .status_code
            .to_string()
            .bold()
            .cyan();
        let line3 = self.status_msg.as_str().bold().cyan();
        let temp = if total_correct > 0 && total_correct == total_testcases {
            vec![line1.into(), line2, ", Msg: ".into(), line3, " ✅".into()]
        }
        else {
            vec![line1.into(), line2, ", Msg: ".into(), line3]
        };

        let mut status_msg_id = vec![
            temp.into(),
            vec!["  • Lang: ".into(), self.pretty_lang.as_str().bold().cyan()].into(),
        ];

        if !self.question_id.is_empty() {
            status_msg_id.push(
                vec![
                    "  • Question ID: ".into(),
                    self.question_id.as_str().bold().cyan(),
                ]
                .into(),
            );
        }
        status_msg_id
    }
    pub fn end_tui_text(&self) -> Vec<Line> {
        let mut status_msg_id = vec![];
        if !self.last_testcase.is_empty() {
            let lines = self
                .last_testcase
                .split('\n')
                .map(|v| vec![v.bold().cyan()].into());
            let mut last_case = vec![vec!["  • Last Testcases: ".into()].into()];
            last_case.extend(lines);
            status_msg_id.extend(last_case);
        }
        if !self.full_compile_error.is_empty() {
            let c_err = self
                .compile_error
                .split('\n')
                .map(|v| -> Line<'_> { v.into() });
            let full_c_err = self
                .full_compile_error
                .split('\n')
                .map(|v| -> Line<'_> { v.into() });
            let mut compile_err = vec!["  • Compile Error:".into()];
            compile_err.extend(full_c_err);
            compile_err.extend(c_err);

            status_msg_id.extend(compile_err);
        }
        if !self.full_runtime_error.is_empty() {
            let r_err = self
                .runtime_error
                .split('\n')
                .map(|v| -> Line<'_> { v.into() });
            let full_r_err = self
                .full_runtime_error
                .split('\n')
                .map(|v| -> Line<'_> { v.into() });
            let mut runtime_err = vec!["  • Runtime Error:".into()];
            runtime_err.extend(full_r_err);
            runtime_err.extend(r_err);

            status_msg_id.extend(runtime_err);
        }
        if !self.code_answer.is_empty() {
            let y_ans = self
                .code_answer
                .iter()
                .map(|v| -> Line<'_> { format!("    • {v}").into() });
            let mut your_ans = vec!["  • Your Answer:".into()];
            your_ans.extend(y_ans);

            status_msg_id.extend(your_ans);
        }
        if !self.expected_code_answer.is_empty() {
            let c_ans1 = self
                .expected_code_answer
                .iter()
                .map(|v| -> Line<'_> { format!("    • {}", v).into() });
            let mut correct_ans = vec!["  • Correct Answer:".into()];
            correct_ans.extend(c_ans1);

            status_msg_id.extend(correct_ans);
        }
        // seem default is `vec![""]`
        if !self.std_output_list.is_empty() && !self.std_output_list[0].is_empty() {
            let std_output = self
                .std_output_list
                .iter()
                .map(|v| -> Line<'_> { format!("    • {v}").into() });
            let mut stdout_ans = vec!["  • Std Output:".into()];
            stdout_ans.extend(std_output);

            status_msg_id.extend(stdout_ans);
        }
        status_msg_id
    }
}

impl Render for RunResult {
    fn to_md_str(&self, _with_env: bool) -> String {
        let total_testcases = self.total_testcases();
        let total_correct = self.total_correct();

        let mut status_id_lang = if total_testcases == total_correct && total_correct > 0 {
            format!(
                "\
                # Status Code: {scode}, Msg: {msg} ✅\n* Lang: {lang}\n",
                scode = self.status_code,
                msg = self.status_msg,
                lang = self.pretty_lang,
            )
        }
        else {
            format!(
                "\
                # Status Code: {scode}, Msg: {msg}\n* Lang: {lang}\n",
                scode = self.status_code,
                msg = self.status_msg,
                lang = self.pretty_lang,
            )
        };
        if self.full_runtime_error.is_empty() && self.full_compile_error.is_empty() {
            let total_test_case = format!(
                "\
                * Total correct: {}\n* Total Testcases: {}\n",
                total_correct, total_testcases,
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
            if let Some(ercentile) = self.runtime_percentile {
                run_time.push_str(&format!(
                    "\
                * Fast Than: {}%\n",
                    ercentile
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
            if let Some(memory_perc) = self.memory_percentile {
                run_memory.push_str(&format!("* Memory Low Than: {}%\n", memory_perc));
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
                    .fold(String::new(), |acc, v| acc + &format!("    * {}\n", v))
            );

            status_id_lang.push_str(&your_answer);
        }
        if !self.expected_code_answer.is_empty() {
            let corr_answer = format!(
                "\
                * Correct Answer: \n{}\n",
                self.expected_code_answer
                    .iter()
                    .fold(String::new(), |acc, v| acc + &format!("    * {}\n", v))
            );
            status_id_lang.push_str(&corr_answer);
        }
        // seem default is `vec![""]`
        if !self.std_output_list.is_empty() && !self.std_output_list[0].is_empty() {
            let out_put = self.std_output_list.join("\n");
            let head = format!(
                "\
                * Std Output:\n{}\n",
                out_put
            );
            status_id_lang.push_str(&head);
        }

        status_id_lang
    }

    #[cfg(feature = "ratatui")]
    fn to_para_vec(&self) -> Vec<Line> {
        let total_testcases = self.total_testcases();
        let total_correct = self.total_correct();

        let mut status_msg_id = self.start_tui_text();

        // make it meaning
        if total_testcases > 0
            && self.full_runtime_error.is_empty()
            && self.full_compile_error.is_empty()
        {
            let total_correct_test_case = vec![
                vec![
                    "  • Total correct: ".into(),
                    total_correct.to_string().bold().cyan(),
                ]
                .into(),
                vec![
                    "  • Total Testcases: ".into(),
                    total_testcases
                        .to_string()
                        .bold()
                        .cyan(),
                ]
                .into(),
            ];

            status_msg_id.extend(total_correct_test_case);
        }
        if !self.status_memory.is_empty() {
            let mut mem_time = vec![vec![
                "  • Memory: ".into(),
                self.status_memory
                    .as_str()
                    .bold()
                    .cyan(),
            ]
            .into()];
            if let Some(percentile) = self.memory_percentile {
                mem_time.push(
                    vec![
                        "  • Memory Low Than: ".into(),
                        percentile.to_string().bold().cyan(),
                        "%".into(),
                    ]
                    .into(),
                );
            }
            mem_time.push(
                vec![
                    "  • Runtime: ".into(),
                    self.status_runtime
                        .as_str()
                        .bold()
                        .cyan(),
                ]
                .into(),
            );
            if let Some(perc) = self.runtime_percentile {
                mem_time.push(
                    vec![
                        "  • Fast Than: ".into(),
                        perc.to_string().bold().cyan(),
                        "%".into(),
                    ]
                    .into(),
                );
            }

            status_msg_id.extend(mem_time);
        }

        let end = self.end_tui_text();
        status_msg_id.extend(end);

        status_msg_id
    }
}
