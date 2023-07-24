pub mod cache;
pub mod config;
pub mod entities;
pub mod leetcode;
pub mod problem_detail;
pub mod render;

#[cfg(test)]
mod tests {

    use super::*;
    use miette::Result;

    #[test]
    fn get_all_pbs_works() -> Result<()> {
        tokio_test::block_on(
            tokio_test::block_on(cache::Cache::new())?.get_all_problems(),
        )?;
        Ok(())
    }

    #[test]
    fn db_dir() {
        let mut a = dirs::cache_dir().unwrap();
        println!(r##"(| a |) -> {:#?}"##, a);
        a.push("leetcode-cn-cli/leetcode.db");
        println!(r##"(| a |) -> {:#?}"##, a);
        let a = a.parent().unwrap();
        println!(r##"(| a |) -> {:#?}"##, a);
    }

    #[test]
    fn get_conf_work() -> Result<()> {
        use crate::config::read_config;
        let a = tokio_test::block_on(read_config::get_user_conf())?;
        dbg!(&a);
        let _a = tokio_test::block_on(read_config::gen_default_conf(false))?;
        Ok(())
    }

    #[test]
    fn database() {}

    #[test]
    fn get_qs_detail_work() -> Result<(), miette::Error> {
        let a = tokio_test::block_on(leetcode::LeetCode::new())?;
        let text = tokio_test::block_on(a.get_problem_detail(
            "algorithms".to_string(),
            3,
            false,
        ))?
        .translated_content
        .unwrap();
        println!("{}", text);
        let text = tokio_test::block_on(a.get_problem_detail(
            "algorithms".to_string(),
            2,
            false,
        ))?
        .translated_content
        .unwrap();
        println!("{}", text);

        Ok(())
    }

    #[test]
    fn render_md() -> Result<(), miette::Error> {
        let a = tokio_test::block_on(leetcode::LeetCode::new())?;
        let id = 1;
        let pb_dt = tokio_test::block_on(a.get_problem_detail(
            "algorithms".to_string(),
            id,
            false,
        ))?;
        let text = pb_dt.translated_content.unwrap_or(
            pb_dt
                .content
                .unwrap_or("不存在".to_string()),
        );

        let text = text.as_str().replace("\\n", "");
        // println!("{}", text);

        use crate::render::*;
        let text = from_html_to_md(&text);
        let text = id.to_string() + "\n\n---\n" + &text + "---";
        // println!("{}", text);
        render_md_str(&text)?;
        Ok(())
    }

    #[test]
    fn gen_df_cf_work() -> Result<()> {
        use crate::config::read_config;
        tokio_test::block_on(read_config::gen_default_conf(true))?;
        tokio_test::block_on(read_config::get_user_conf())?;
        Ok(())
    }

    #[test]
    fn def_value() {
        let df = serde_json::Value::default();
        println!(r##"(| df |) -> {:#?}"##, df);
        /// 嵌套字段
        #[derive(Debug, Default)]
        pub struct Param {
            pub name: String,
            pub r#type: String,
            pub dealloc: bool,
        }
        let param = Param {
            ..Default::default()
        };
        println!(r##"(| param |) -> {:#?}"##, param);
    }
}
