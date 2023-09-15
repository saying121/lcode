use std::fmt::Display;

use serde::{Deserialize, Serialize};
use tabled::{
    builder::Builder,
    settings::{style::Style, themes::ColumnNames},
};

use crate::config::global::glob_user_config;

#[derive(Deserialize, Serialize, Debug)]
pub struct SubmissionList {
    #[serde(alias = "lastKey")]
    pub(crate) last_key: String,
    #[serde(alias = "hasNext")]
    pub(crate) has_next: bool,
    pub(crate) submissions: Vec<list_nest::Submission>,
}

impl Display for SubmissionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let user = glob_user_config();

        let mut subs = vec![];
        let mut temp = vec![];

        for i in 0..user.column {
            temp.push(i.to_string());
        }

        subs.push(temp.clone());
        temp.clear();

        for submission in &self.submissions {
            temp.push(submission.to_string());
            if temp.len() >= user.column {
                subs.push(temp.clone());
                temp.clear();
            }
        }
        if !temp.is_empty() {
            subs.push(temp.clone());
        }

        let mut table = Builder::from(subs).build();
        table
            .with(Style::modern())
            .with(ColumnNames::default());

        format!("{}", table).fmt(f)
    }
}

pub mod list_nest {
    use std::fmt::Display;

    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Submission {
        pub id: String,
        pub title: String,
        pub status: Option<String>,
        #[serde(alias = "statusDisplay")]
        pub status_display: Option<String>,
        pub lang: String,
        #[serde(alias = "langName")]
        pub lang_name: String,
        pub runtime: String,
        pub timestamp: String,
        pub url: String,
        #[serde(alias = "isPending")]
        pub is_pending: String,
        pub memory: String,
        #[serde(alias = "submissionComment")]
        pub submission_comment: Option<String>,
    }

    impl Display for Submission {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            format!(
                "• Title: {title} \n\
                • ID: {id} \n\
                • Lang: {lang} \n\
                • Status: {status} \n\
                • Runtime: {tim} \n\
                • Memory: {mem} \n\
                • Comment: {cmt} \n\
                ",
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
}
