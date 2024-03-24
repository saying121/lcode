use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use lcode_config::config::{global::G_DATABASE_PATH, read_config, user_nest::Suffix};
use leetcode_api::{leetcode::IdSlug, render::Render};
use miette::{IntoDiagnostic, Result};
use tokio::{fs, time::Instant};

use crate::{
    editor::{edit_config, open, CodeTestFile},
    fuzzy_search::select_a_question,
    glob_leetcode, mytui,
};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(
        alias = "e",
        about = format!("Edit `{cd}` or `{ts}`, default edit `{cd}` {alias}",
                    cd = "code".bold(),
                    ts = "test cases".bold(),
                    alias = "[ alias: e ]".bold()
                )
        )
    ]
    Edit(EditArgs),
    #[command(
        alias = "f",
        about = format!("Interact select a question (fuzzy search), default view detail {}", "[ alias: f ]".bold())
    )]
    Fzy(InterArgs),
    #[command(alias = "D", about = format!("View a question detail {}", "[ alias: D ]".bold()))]
    Detail(DetailArgs),
    #[command(alias = "S", about = format!("Syncanhronize leetcode info {}","[ alias: S ]".bold()))]
    Sync(Force),
    #[command(alias = "t", about = format!("Test your code {}", "[ alias: t ]".bold()))]
    Test(SubTestArgs),
    #[command(alias = "st", about = format!("Submit your code {}", "[ alias: st ]".bold()))]
    Submit(SubTestArgs),
    #[command(alias = "sl", about = format!("Get submit list {}", "[ alias: sl ]".bold()))]
    Sublist(SubTestArgs),
    #[command(alias = "g", about = format!("Generate a config, will also be automatically generated at runtime {}","[ alias: g ]".bold()))]
    Gencon(GenArgs),
    #[command(alias = "T", about = format!("Open Tui {}", "[ alias: T ]".bold()))]
    Tui,
    #[command(alias = "C", about = format!("Edit config {}", "[ alias: C ]".bold()))]
    Config,
    #[command(about = format!("Give the project a star"))]
    Star,
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
    id:    u32,
    #[arg(short, long, help = "Force update question's information")]
    force: bool,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct Force {
    #[arg(short, long, help = "Delete database for full re-sync")]
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
    #[command(about = "Edit code(default)")]
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

/// Cli runner
pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config => edit_config().await?,
        Commands::Star => crate::star(),
        Commands::Tui => Box::pin(mytui::run()).await?,
        Commands::Sublist(args) => {
            let res = glob_leetcode()
                .await
                .all_submit_res(IdSlug::Id(args.id))
                .await?;
            println!("{}", res);
        },
        Commands::Gencon(args) => {
            read_config::gen_config(if args.cn { Suffix::Cn } else { Suffix::Com })?;
        },

        Commands::Submit(args) => {
            let (_, res) = glob_leetcode()
                .await
                .submit_code(IdSlug::Id(args.id))
                .await?;
            res.render_with_mdcat();
        },
        Commands::Test(args) => {
            let (_, res) = glob_leetcode()
                .await
                .test_code(IdSlug::Id(args.id))
                .await?;
            res.render_with_mdcat();
        },
        Commands::Sync(args) => {
            if args.force {
                fs::remove_file(&*G_DATABASE_PATH)
                    .await
                    .into_diagnostic()?;
            }
            let start = Instant::now();
            println!("Waiting ……");

            glob_leetcode()
                .await
                .sync_problem_index()
                .await?;

            println!(
                "Syncanhronize Done, spend: {}s",
                (Instant::now() - start).as_secs_f64()
            );
        },
        Commands::Edit(args) => match args.command {
            Some(cmd) => match cmd {
                CoT::Code(id) => open(IdSlug::Id(id.input), CodeTestFile::Code).await?,
                CoT::Test(id) => open(IdSlug::Id(id.input), CodeTestFile::Test).await?,
            },
            None => match args.id {
                Some(id) => open(IdSlug::Id(id.input), CodeTestFile::Code).await?,
                None => println!("please give info"),
            },
        },
        Commands::Detail(args) => {
            let qs = glob_leetcode()
                .await
                .get_qs_detail(IdSlug::Id(args.id), args.force)
                .await?;
            qs.render_with_mdcat();
        },
        Commands::Fzy(args) => match args.command {
            Some(ag) => match ag {
                DetailOrEdit::Detail(detail_args) => {
                    let id = select_a_question().await?;

                    if id == 0 {
                        return Ok(());
                    }

                    let qs = glob_leetcode()
                        .await
                        .get_qs_detail(IdSlug::Id(id), detail_args.force)
                        .await?;
                    qs.render_with_mdcat();
                },
                DetailOrEdit::Edit => {
                    let id = select_a_question().await?;

                    if id == 0 {
                        return Ok(());
                    }

                    open(IdSlug::Id(id), CodeTestFile::Code).await?;
                },
            },
            None => {
                let id = select_a_question().await?;

                if id == 0 {
                    return Ok(());
                }

                let qs = glob_leetcode()
                    .await
                    .get_qs_detail(IdSlug::Id(id), false)
                    .await?;
                qs.render_with_mdcat();
            },
        },
    };

    Ok(())
}
