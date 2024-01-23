use question::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Problems {
    pub user_name:         String,
    pub num_solved:        u32,
    pub num_total:         u32,
    pub ac_easy:           u32,
    pub ac_medium:         u32,
    pub ac_hard:           u32,
    pub stat_status_pairs: Vec<QsIndex>,
}

/// base info of question
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct QsIndex {
    #[serde(default)]
    pub stat:       Stat,
    #[serde(default)]
    pub status:     Option<String>,
    #[serde(default)]
    pub difficulty: Difficulty,
    #[serde(default)]
    pub paid_only:  bool,
    #[serde(default)]
    pub is_favor:   bool,
    #[serde(default)]
    pub frequency:  u32,
    #[serde(default)]
    pub progress:   u32,
}

pub mod question {
    use lcode_config::config::{global::G_USER_CONFIG, user_nest::Suffix};
    use serde::{Deserialize, Deserializer, Serialize};

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct Stat {
        pub question_id:          u32,
        #[serde(rename = "question__title")]
        pub question_title:       String,
        #[serde(rename = "question__title_slug")]
        pub question_title_slug:  String,
        #[serde(rename = "question__hide")]
        pub question_hide:        bool,
        pub total_acs:            u32,
        pub total_submitted:      u32,
        #[serde(default, deserialize_with = "my_id_deserialize")]
        pub frontend_question_id: String,
    }

    fn my_id_deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let res = match G_USER_CONFIG.config.url_suffix {
            Suffix::Cn => String::deserialize(deserializer)?,
            Suffix::Com => u32::deserialize(deserializer)?.to_string(),
        };
        Ok(res)
    }

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct Difficulty {
        pub level: u32,
    }
}
