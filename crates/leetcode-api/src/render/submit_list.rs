use std::{fmt::Display, mem};

use lcode_config::global::G_USER_CONFIG;
use tabled::{
    builder::Builder,
    settings::{style::Style, themes::ColumnNames},
};

use crate::leetcode::resps::submit_list::{Submission, SubmissionList};

impl Display for SubmissionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.submissions.is_empty() {
            return "No information".fmt(f);
        }

        let mut subs = vec![];
        let mut temp = Vec::with_capacity(
            G_USER_CONFIG
                .config
                .column
                .min(self.submissions.len()),
        );

        subs.push(mem::take(&mut temp));

        for submission in &self.submissions {
            temp.push(submission.to_string());
            if temp.len() >= G_USER_CONFIG.config.column {
                subs.push(mem::take(&mut temp));
            }
        }
        if !temp.is_empty() {
            subs.push(temp);
        }

        let mut table = Builder::from(subs).build();
        table
            .with(Style::modern())
            .with(ColumnNames::default());

        format!("{}", table).fmt(f)
    }
}

impl Display for Submission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(
            "• Title: {title} \n• ID: {id} \n• Lang: {lang} \n• Status: {status} \n• Runtime: \
             {tim} \n• Memory: {mem} \n• Comment: {cmt} \n",
            cmt = self
                .submission_comment
                .as_deref()
                .unwrap_or_default(),
            id = self.id,
            title = self.title,
            lang = self.lang_name,
            status = self
                .status_display
                .as_deref()
                .unwrap_or_default(),
            tim = self.runtime,
            mem = self.memory,
        )
        .fmt(f)
    }
}
