use crate::{
    config::read_config,
    editor::{edit, CodeTestFile},
    fuzzy_search::select_a_question,
    leetcode::{IdSlug, LeetCode},
    render::{render_qs_to_tty, render_str},
};
use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use tokio::time::Instant;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(alias = "e", about = format!("Edit `{cd}` or `{ts}`, default edit `{cd}`", cd = "code".bold(), ts = "test cases".bold()))]
    Edit(EditArgs),
    #[command(
        alias = "f",
        about = "Interact select a question edit it or view details (fuzzy search), default view detail"
    )]
    Fzy(InterArgs),
    #[command(alias = "d", about = "View a question detail")]
    Detail(DetailArgs),
    #[command(alias = "sy", about = "Syncanhronize leetcode index info")]
    Sync,
    #[command(alias = "st", about = "Submit your code")]
    Submit(SubTestArgs),
    #[command(alias = "sl", about = "Get submit list")]
    Sublist(SubTestArgs),
    #[command(alias = "t", about = format!("Test your code, you can use `{}` subcommand to edit your test case","edit test".bold()))]
    Test(SubTestArgs),
    #[command(alias = "g", about = format!("Generate a config, will also be automatically generated at runtime"))]
    Gencon(GenArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct GenArgs {
    #[arg(short, long)]
    cn: bool,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct SubTestArgs {
    id: u32,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct InterArgs {
    #[command(subcommand)]
    command: Option<DetailOrEdit>,
}

#[derive(Debug, Subcommand)]
enum DetailOrEdit {
    #[command(about = "View detail")]
    Detail(DetailArgs),
    #[command(about = "Edit code")]
    Edit,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct DetailArgs {
    #[arg(help = "Force update question's information")]
    id: u32,
    #[arg(short, long, help = "Force update question's information")]
    force: bool,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct EditArgs {
    #[command(subcommand)]
    command: Option<CoT>,

    #[command(flatten, help = "Id  of the be edited question, default edit it")]
    id: Option<EditCodeArgs>,
}

#[derive(Debug, Subcommand)]
enum CoT {
    #[command(about = "Edit code")]
    Code(EditCodeArgs),
    #[command(about = "Edit test case")]
    Test(EditCodeArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct EditCodeArgs {
    #[arg(help = "Question id")]
    input: u32,
}

pub async fn run() -> miette::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sublist(args) => {
            let leetcode = LeetCode::new().await?;
            let res = leetcode
                .all_submit_res(IdSlug::Id(args.id))
                .await?;
            println!("{}", res);
        }
        Commands::Gencon(args) => {
            let tongue = match args.cn {
                true => "cn",
                false => "en",
            };
            read_config::gen_default_conf(tongue)?;
        }
        Commands::Submit(args) => {
            let leetcode = LeetCode::new().await?;
            let (_, res) = leetcode
                .submit_code(IdSlug::Id(args.id))
                .await?;
            render_str(res.to_string())?
        }
        Commands::Test(args) => {
            let leetcode = LeetCode::new().await?;
            let (_, res) = leetcode
                .test_code(IdSlug::Id(args.id))
                .await?;
            render_str(res.to_string())?
        }
        Commands::Sync => {
            let start = Instant::now();
            let leetcode = LeetCode::new().await?;
            leetcode
                .sync_problem_index()
                .await?;
            let end = Instant::now();
            println!(
                "Syncanhronize Done, spend: {}s",
                (end - start).as_secs_f64()
            );
        }
        Commands::Edit(args) => match args.command {
            Some(cmd) => match cmd {
                CoT::Code(id) => edit(IdSlug::Id(id.input), CodeTestFile::Code).await?,
                CoT::Test(id) => edit(IdSlug::Id(id.input), CodeTestFile::Test).await?,
            },
            None => match args.id {
                Some(id) => edit(IdSlug::Id(id.input), CodeTestFile::Code).await?,
                None => println!("please give info"),
            },
        },
        Commands::Detail(args) => {
            let leetcode = LeetCode::new().await?;
            let qs = leetcode
                .get_problem_detail(IdSlug::Id(args.id), args.force)
                .await?;
            render_qs_to_tty(qs)?;
        }
        Commands::Fzy(args) => match args.command {
            Some(ag) => match ag {
                DetailOrEdit::Detail(detail_args) => {
                    let id = select_a_question().await?;
                    let leetcode = LeetCode::new().await?;
                    let qs = leetcode
                        .get_problem_detail(IdSlug::Id(id), detail_args.force)
                        .await?;
                    render_qs_to_tty(qs)?;
                }
                DetailOrEdit::Edit => {
                    let id = select_a_question().await?;
                    edit(IdSlug::Id(id), CodeTestFile::Code).await?
                }
            },
            None => {
                let id = select_a_question().await?;
                let leetcode = LeetCode::new().await?;
                let qs = leetcode
                    .get_problem_detail(IdSlug::Id(id), false)
                    .await?;
                render_qs_to_tty(qs)?;
            }
        },
    };

    Ok(())
}
