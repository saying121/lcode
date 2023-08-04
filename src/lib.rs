pub mod config;
pub mod editor;
pub mod entities;
pub mod fuzzy_search;
pub mod leetcode;
pub mod render;
pub mod storage;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        editor::edit,
        leetcode::{IdSlug, LeetCode},
    };
    use miette::Result;
    use tokio_test::block_on;
    use tracing_error::ErrorLayer;
    use tracing_subscriber::{
        filter::EnvFilter, fmt, prelude::__tracing_subscriber_SubscriberExt,
        util::SubscriberInitExt, Registry,
    };

    #[test]
    fn get_all_pbs_works() -> Result<()> {
        // use tracing_subscriber::filter::{EnvFilter, LevelFilter};
        // let tmp = EnvFilter::builder()
        //     .with_default_directive(LevelFilter::ERROR.into())
        //     .parse_lossy("error");
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
        let formatting_layer = fmt::layer()
            .pretty()
            .with_writer(std::io::stderr);
        Registry::default()
            .with(env_filter)
            .with(ErrorLayer::default())
            .with(formatting_layer)
            .init();
        block_on(block_on(LeetCode::new())?.sync_problem_index())?;
        Ok(())
    }

    #[test]
    fn query_question_work() -> Result<()> {
        use crate::{leetcode::IdSlug, storage::query_question};
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
        let formatting_layer = fmt::layer()
            .pretty()
            .with_writer(std::io::stderr);
        Registry::default()
            .with(env_filter)
            .with(ErrorLayer::default())
            .with(formatting_layer)
            .init();
        let a = block_on(query_question::get_question_index(IdSlug::Id(0)))?;
        println!(r##"(| 0 a |) -> {:#?}"##, a);
        let a = block_on(query_question::get_question_index_exact(IdSlug::Id(1)))?;
        println!(r##"(| a |) -> {:#?}"##, a);

        let a = tokio_test::block_on(query_question::get_question_index(IdSlug::Slug(
            "two-sum".to_string(),
        )))?;
        println!(r##"(| a |) -> {:#?}"##, a);

        Ok(())
    }

    #[test]
    fn submit_work() -> Result<()> {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
        let formatting_layer = fmt::layer()
            .pretty()
            .with_writer(std::io::stderr);
        Registry::default()
            .with(env_filter)
            .with(ErrorLayer::default())
            .with(formatting_layer)
            .init();

        let a = block_on(leetcode::LeetCode::new())?;
        let res = block_on(a.submit_code(IdSlug::Id(1)));
        match res {
            Ok(v) => println!(r##"(| v |) -> {:#?}"##, v),
            Err(err) => println!("{}", err),
        };

        Ok(())
    }

    #[test]
    fn test_work() -> Result<()> {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
        let formatting_layer = fmt::layer()
            .pretty()
            .with_writer(std::io::stderr);
        Registry::default()
            .with(env_filter)
            .with(ErrorLayer::default())
            .with(formatting_layer)
            .init();

        let a = block_on(leetcode::LeetCode::new())?;
        let _res = block_on(a.test_code(IdSlug::Id(1)))?;

        Ok(())
    }

    #[test]
    fn get_qs_detail_work() -> Result<(), miette::Error> {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
        let formatting_layer = fmt::layer()
            .pretty()
            .with_writer(std::io::stderr);
        Registry::default()
            .with(env_filter)
            .with(ErrorLayer::default())
            .with(formatting_layer)
            .init();

        let a = block_on(leetcode::LeetCode::new())?;
        let question = block_on(a.get_problem_detail(IdSlug::Id(1), false))?;
        println!(r##"(| qsdetail |) -> {:#?}"##, question);
        //
        // let questions = block_on(
        //     a.get_problem_detail(IdSlug::Slug("zigzag-conversion".to_owned()), false),
        // )?;
        //
        // for qs in questions {
        //     println!("{}", qs.content.unwrap_or_default());
        //     println!(
        //         "{}",
        //         qs.translated_content
        //             .unwrap_or_default()
        //     );
        // }

        Ok(())
    }

    #[test]
    fn get_conf_work() -> Result<()> {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
        let formatting_layer = fmt::layer()
            .pretty()
            .with_writer(std::io::stderr);
        Registry::default()
            .with(env_filter)
            .with(ErrorLayer::default())
            .with(formatting_layer)
            .init();

        use crate::config::read_config;
        let _a = block_on(read_config::gen_default_conf(false, "cn"))?;
        let a = block_on(read_config::get_user_conf())?;
        println!(r##"(| a |) -> {:#?}"##, a);
        Ok(())
    }

    #[test]
    fn render_md() -> Result<(), miette::Error> {
        let a = block_on(leetcode::LeetCode::new())?;
        let id = 1;
        let qs = block_on(a.get_problem_detail(IdSlug::Id(id), false))?;

        let text = qs.translated_content.unwrap_or(
            qs.content
                .unwrap_or("not exists".to_owned()),
        );
        let text = text
            .as_str()
            .trim_matches('"')
            .replace("\\n", "");
        println!("html: \n{}", text);

        use crate::render::*;
        let text = from_html_to_md(&text);
        let text = id.to_string() + "\n\n---\n" + &text + "\n---";
        println!("{}", text);
        render_md_str(&text)?;

        Ok(())
    }

    #[test]
    fn edit_work() -> Result<()> {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
        let formatting_layer = fmt::layer()
            .pretty()
            .with_writer(std::io::stderr);
        Registry::default()
            .with(env_filter)
            .with(ErrorLayer::default())
            .with(formatting_layer)
            .init();

        block_on(edit(IdSlug::Id(1), editor::CodeTestFile::Code))?;
        block_on(edit(IdSlug::Id(1), editor::CodeTestFile::Test))?;
        Ok(())
    }
}
