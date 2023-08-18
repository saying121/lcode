pub mod cli;
pub mod config;
pub mod editor;
pub mod entities;
pub mod fuzzy_search;
pub mod leetcode;
pub mod mytui;
pub mod render;
pub mod storage;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        config::global::{global_leetcode, global_user_config},
        editor::edit,
        fuzzy_search::select_a_question,
        leetcode::IdSlug,
        render::*,
    };
    use miette::{Error, Result};

    use tokio_test::block_on;
    use tracing_error::ErrorLayer;
    use tracing_subscriber::{
        filter::EnvFilter, fmt, prelude::__tracing_subscriber_SubscriberExt,
        util::SubscriberInitExt, Registry,
    };

    #[test]
    fn mytui_work() -> Result<()> {
        block_on(mytui::run())?;
        Ok(())
    }

    #[test]
    fn render_html() -> Result<(), Error> {
        let a = global_leetcode();
        let qs = block_on(a.get_problem_detail(IdSlug::Id(1), false))?;

        println!(r##"(| temp |) -> {:#?}"##, qs.to_tui_mdvec(80));

        Ok(())
    }

    #[test]
    fn select_work() -> Result<()> {
        let id = block_on(select_a_question())?;
        if id == 0 {
            return Ok(());
        }
        println!("{}", id);

        let a = global_leetcode();
        let qs = block_on(a.get_problem_detail(IdSlug::Id(id), false))?;
        render_qs_to_tty(qs)?;
        Ok(())
    }

    #[test]
    fn index_display_work() -> Result<()> {
        use crate::storage::query_question;
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

        let idx = tokio_test::block_on(query_question::query_all_index())?;
        println!("{:#?}", idx[1]);
        for i in 0..5 {
            println!("{}", idx[i]);
        }
        let length = idx.len();
        println!("{}", idx[length - 1]);
        println!("{}", idx[length - 2]);
        println!("{}", idx[length - 3]);

        Ok(())
    }

    #[test]
    fn get_all_pbs_works() -> Result<()> {
        // let env_filter =
        //     EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
        // let formatting_layer = fmt::layer()
        //     .pretty()
        //     .with_writer(std::io::stderr);
        // Registry::default()
        //     .with(env_filter)
        //     .with(ErrorLayer::default())
        //     .with(formatting_layer)
        //     .init();
        block_on(global_leetcode().sync_problem_index())?;
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

        let a = global_leetcode();
        let res = block_on(a.submit_code(IdSlug::Id(1)));
        match res {
            Ok(v) => {
                let (_, res) = v;
                println!("{}", res);
                render_str(res.to_string())?;
            }
            Err(err) => println!("{}", err),
        };

        Ok(())
    }

    #[test]
    fn get_submit_list() -> Result<()> {
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

        let a = global_leetcode();
        let res = block_on(a.all_submit_res(IdSlug::Id(1)))?;
        println!("{}", res);
        // render_str(res.to_string())?;
        // let res = get_rendered_str(res.to_string(), 30, 10)?;
        // println!("{}", res);

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

        let a = global_leetcode();
        if let Ok((_, res)) = block_on(a.test_code(IdSlug::Id(1))) {
            println!(r##"(| res |) -> {} "##, res);
            render_str(res.to_string())?;
        }

        Ok(())
    }

    #[test]
    fn get_qs_detail_work() -> Result<(), Error> {
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

        let a = global_leetcode();
        let question = block_on(a.get_problem_detail(IdSlug::Id(1), false))?;
        println!(r##"(| qsdetail |) -> {:#?}"##, question);

        Ok(())
    }

    #[test]
    #[should_panic]
    fn get_qs_detail_work1() {
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

        let a = global_leetcode();
        let question = block_on(a.get_problem_detail(IdSlug::Id(0), false)).unwrap();
        println!(r##"(| qsdetail |) -> {:#?}"##, question);
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
        let _a = read_config::gen_default_conf("cn")?;
        // let a = read_config::get_user_conf()?;
        // println!(r##"(| a |) -> {:#?}"##, a);
        let a = global_user_config();
        println!(r##"(| a |) -> {:#?}"##, a);
        Ok(())
    }

    #[test]
    fn render_md_terminal() -> Result<(), Error> {
        let a = global_leetcode();
        let id = 1;
        let qs = block_on(a.get_problem_detail(IdSlug::Id(id), false))?;

        use crate::render::*;
        render_qs_to_tty(qs)?;

        Ok(())
    }

    #[test]
    fn render_md_str() -> Result<(), Error> {
        let a = global_leetcode();
        let id = 1;
        let qs = block_on(a.get_problem_detail(IdSlug::Id(id), false))?;

        use render::Render;
        let a = qs.to_rendered_str(80, 80)?;
        println!("{}", a);

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

        block_on(edit(IdSlug::Id(1000570), editor::CodeTestFile::Code))?;
        block_on(edit(IdSlug::Id(1000570), editor::CodeTestFile::Test))?;
        Ok(())
    }
}
