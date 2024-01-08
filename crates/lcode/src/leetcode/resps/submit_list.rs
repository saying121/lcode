use std::fmt::Display;

use lcode_config::config::global::USER_CONFIG;
use serde::{Deserialize, Serialize};
use tabled::{
    builder::Builder,
    settings::{style::Style, themes::ColumnNames},
};

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct SubmissionData {
    #[serde(default)]
    pub data: SubmissionDataInner,
}
#[derive(Default, Deserialize, Serialize, Debug)]
pub struct SubmissionDataInner {
    #[serde(default, alias = "submissionList")]
    pub submission_list: SubmissionList,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct SubmissionList {
    #[serde(default, alias = "lastKey")]
    pub(crate) last_key:    Option<String>,
    #[serde(default, alias = "hasNext")]
    pub(crate) has_next:    bool,
    #[serde(default)]
    pub(crate) submissions: Vec<list_nest::Submission>,
}

impl Display for SubmissionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.submissions.is_empty() {
            return "No information".fmt(f);
        }

        let mut subs = vec![];
        let mut temp = Vec::with_capacity(
            USER_CONFIG
                .config
                .column
                .min(self.submissions.len()),
        );

        for i in 0..USER_CONFIG
            .config
            .column
            .min(self.submissions.len())
        {
            temp.push(i.to_string());
        }

        subs.push(temp.clone());
        temp.clear();

        for submission in &self.submissions {
            temp.push(submission.to_string());
            if temp.len() >= USER_CONFIG.config.column {
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
        #[serde(default)]
        pub id:                 String,
        #[serde(default)]
        pub title:              String,
        #[serde(default)]
        pub status:             Option<String>,
        #[serde(default, alias = "statusDisplay")]
        pub status_display:     Option<String>,
        #[serde(default)]
        pub lang:               String,
        #[serde(default, alias = "langName")]
        pub lang_name:          String,
        #[serde(default)]
        pub runtime:            String,
        #[serde(default)]
        pub timestamp:          String,
        #[serde(default)]
        pub url:                String,
        #[serde(default, alias = "isPending")]
        pub is_pending:         String,
        #[serde(default)]
        pub memory:             String,
        #[serde(default, alias = "submissionComment")]
        pub submission_comment: Option<String>,
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
}
