use clap::{Args, Parser, Subcommand};
use colored::Colorize;

use crate::{
    editor::{edit, CodeTestFile},
    fuzzy_search::select_a_question,
    leetcode::{IdSlug, LeetCode},
    render::render_qs_to_tty,
};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = format!("edit `{cd}` or `{ts}`, default edit `{cd}`", cd = "code".bold(), ts = "test cases".bold()))]
    Edit(EditArgs),
    #[command(
        about = "interact select a question edit it or view details (fuzzy search), default view detail"
    )]
    Fzy(InterArgs),
    #[command(about = "view a question detail")]
    Detail(DetailArgs),
    #[command(about = "syncanhronize leetcode index info")]
    Sync,
    #[command(about = "submit your code")]
    Submit(SubTestArgs),
    #[command(about = format!("test your code, you can use `{}` subcommand to edit your test case","edit test".bold()))]
    Test(SubTestArgs),
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
    #[command(about = "view detail")]
    Detail(DetailArgs),
    #[command(about = "edit code")]
    Edit,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct DetailArgs {
    #[arg(help = "force update question's information")]
    id: u32,
    #[arg(short, long, help = "force update question's information")]
    force: bool,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct EditArgs {
    #[command(subcommand)]
    command: Option<CoT>,

    #[command(flatten, help = "id  of the be edited question, default edit it")]
    id: Option<EditCodeArgs>,
}

#[derive(Debug, Subcommand)]
enum CoT {
    #[command(about = "edit code")]
    Code(EditCodeArgs),
    #[command(about = "edit test case")]
    Test(EditCodeArgs),
}

#[derive(Debug, Args)]
struct EditCodeArgs {
    /// question id
    input: u32,
}

pub async fn run() -> miette::Result<()> {
    let cli = Cli::parse();
    let leetcode = LeetCode::new().await?;

    match cli.command {
        Commands::Submit(args) => {
            let (_, res) = leetcode
                .submit_code(IdSlug::Id(args.id))
                .await?;
            println!("{}", res);
        }
        Commands::Test(args) => {
            let (_, res) = leetcode
                .test_code(IdSlug::Id(args.id))
                .await?;
            println!("{}", res);
        }
        Commands::Sync => {
            leetcode
                .sync_problem_index()
                .await?;
            println!("Syncanhronize Done");
        }
        Commands::Edit(v) => match v.command {
            Some(cmd) => match cmd {
                CoT::Code(id) => edit(IdSlug::Id(id.input), CodeTestFile::Code).await?,
                CoT::Test(id) => edit(IdSlug::Id(id.input), CodeTestFile::Test).await?,
            },
            None => match v.id {
                Some(id) => edit(IdSlug::Id(id.input), CodeTestFile::Code).await?,
                None => println!("please give info"),
            },
        },
        Commands::Detail(dt_args) => {
            let qs = leetcode
                .get_problem_detail(IdSlug::Id(dt_args.id), dt_args.force)
                .await?;
            render_qs_to_tty(qs)?;
        }
        Commands::Fzy(args) => match args.command {
            Some(ag) => match ag {
                DetailOrEdit::Detail(detail_args) => {
                    let id = select_a_question().await?;
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
                let qs = leetcode
                    .get_problem_detail(IdSlug::Id(id), false)
                    .await?;
                render_qs_to_tty(qs)?;
            }
        },
    };

    Ok(())
}
