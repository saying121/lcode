use std::fmt::{Display, Write as _};

use lcode_config::global::G_USER_CONFIG;
#[cfg(feature = "ratatui")]
use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span},
};

use super::{to_sub_sup_script, Render};
use crate::leetcode::question::qs_detail::Question;

impl Render for Question {
    fn to_md_str(&self, with_env: bool) -> String {
        let content = if G_USER_CONFIG.config.translate {
            self.translated_content
                .as_deref()
                .unwrap_or_default()
        }
        else {
            self.content
                .as_deref()
                .unwrap_or_default()
        };

        let content = to_sub_sup_script(content)
            .trim_matches('"')
            .replace("\\n", "\n");
        let env_info = self.env_info.to_string();

        // some content are not HTML
        let md_str = if content.contains("<p>") {
            html2text::from_read(content.as_bytes(), 80).map_or(content, |s| s)
        }
        else {
            content
        };
        let t_case = format!("```txt\n{}\n```", self.example_testcases);
        let mut res = format!(
            "{qs}\n## Content\n\n{md}\n---\n\n## Test Case\n\n{test}\n",
            qs = self,
            md = md_str,
            test = t_case
        );

        if !self.hints.is_empty() {
            let join = self.hints.join("\n");
            let hints = html2text::from_read(join.as_bytes(), 80).map_or(join, |s| s);
            res = format!("{}\n\nhints:\n{}\n---\n", res, hints);
        }
        if !self.mysql_schemas.is_empty() {
            let str = format!("\n```sql\n{}\n```\n", self.mysql_schemas.join("\n"));
            res.push_str(&str);
        }
        if with_env {
            let _ = write!(&mut res, "\n## EnvInfo\n\n{}", env_info);
        }
        res
    }

    #[cfg(feature = "ratatui")]
    fn to_para_vec(&self) -> Vec<Line> {
        use scraper::Html;

        let content = if G_USER_CONFIG.config.translate {
            self.translated_content
                .as_deref()
                .unwrap_or_else(|| {
                    self.content
                        .as_deref()
                        .unwrap_or_default()
                })
        }
        else {
            self.content
                .as_deref()
                .unwrap_or_default()
        };

        let content = to_sub_sup_script(content);

        let frag = Html::parse_fragment(&content);
        let res = frag
            .root_element()
            .text()
            .fold(String::new(), |acc, e| acc + e);

        let res = res
            .replace("\\\"", "\"")
            .replace("\\\\", "")
            .replace("\\n", "\n")
            .replace("\\t", "    ")
            .replace("\n\n\n", "\n");

        let res = res
            .trim_matches(|c| c == '"' || c == '\n' || c == ' ')
            .split('\n')
            .map(|v| -> Line<'_> { v.to_owned().into() });

        let topic = self
            .topic_tags
            .iter()
            .map(|v| {
                if G_USER_CONFIG.config.translate {
                    v.translated_name
                        .as_ref()
                        .map_or_else(|| v.name.as_str(), |v| v.as_str())
                }
                else {
                    v.name.as_str()
                }
            })
            .fold(String::new(), |acc, v| {
                if acc.is_empty() {
                    return v.to_owned();
                }
                format!("{}, {}", acc, v)
            });

        let mut res1 = vec![
            vec![
                Span::styled("• ID: ", Style::default()),
                Span::styled(self.question_id.as_str(), Style::default().bold()),
                Span::styled(" | Passing rate: ", Style::default()),
                Span::styled(self.stats.ac_rate.as_str(), Style::default().bold()),
                Span::styled(" | PaidOnly: ", Style::default()),
                Span::styled(self.is_paid_only.to_string(), Style::default().bold()),
                Span::styled(" | Difficulty: ", Style::default()),
                Span::styled(self.difficulty.as_str(), Style::default().bold()),
            ]
            .into(),
            vec![
                Span::styled("• Topic: ", Style::default().bold()),
                Span::styled(topic, Style::default()),
            ]
            .into(),
            vec![
                Span::styled("• Url: ", Style::default()),
                Span::styled(
                    G_USER_CONFIG.urls.get_qs_url(
                        self.qs_slug
                            .as_deref()
                            .unwrap_or_default(),
                    ),
                    Style::default().bold(),
                ),
            ]
            .into(),
            "".into(),
        ];
        res1.extend(res);

        res1
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = if G_USER_CONFIG.config.translate {
            self.translated_title
                .as_ref()
                .map_or(self.title.as_str(), |v| v.as_str())
                .trim_matches('"')
        }
        else {
            self.title.as_str()
        };

        let topic = self
            .topic_tags
            .iter()
            .map(|v| {
                if G_USER_CONFIG.config.translate {
                    return v
                        .translated_name
                        .as_ref()
                        .map_or(v.name.as_str(), |translated_name| translated_name.as_str());
                }
                v.name.as_str()
            })
            .fold(String::new(), |acc, v| {
                if acc.is_empty() {
                    return v.to_owned();
                }
                format!("{}, {}", acc, v)
            });

        format!(
            "# {tit:62}\n\n* ID: [{id:07}]({url}) | Passing rate: {rt:.6} | PaidOnly: {pd:6} | \
             Difficulty: {di}\n* Topic: {tp}\n",
            tit = title,
            id = self.question_id,
            rt = self.stats.ac_rate,
            pd = self.is_paid_only,
            di = self.difficulty,
            tp = topic,
            url = G_USER_CONFIG.urls.get_qs_url(
                self.qs_slug
                    .as_deref()
                    .unwrap_or_default()
            )
        )
        .fmt(f)
    }
}
