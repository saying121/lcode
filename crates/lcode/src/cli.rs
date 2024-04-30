use std::io;

use clap::{Args, Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use colored::Colorize;
use lcode_config::config::{global::G_DATABASE_PATH, read_config, user_nest::Suffix};
use leetcode_api::{leetcode::IdSlug, render::Render};
use miette::{IntoDiagnostic, Result};
use tokio::{fs, time::Instant};

use crate::{
    editor::{CodeTestFile, Editor},
    fuzzy_search::select_a_question,
    glob_leetcode, mytui,
};

#[derive(Debug)]
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,
    #[command(subcommand)]
    command:   Option<Commands>,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_owned(), &mut io::stdout());
}

#[derive(Debug)]
#[derive(Subcommand)]
enum Commands {
    #[command(
        alias = "e",
        about = format!("Edit `{cd}` or `{ts}`, {alias}",
                    cd = "code".bold(),
                    ts = "test cases".bold(),
                    alias = "[ alias: e ]".bold()
                )
        )
    ]
    Edit(EditArgs),
    #[command(
        alias = "f",
        about = format!("Interact select a question (fuzzy search), {}", "[ alias: f ]".bold())
    )]
    Fzy(InterArgs),
    #[command(alias = "D", about = format!("View a question detail {}", "[ alias: D ]".bold()))]
    Detail(DetailArgs),
    #[command(alias = "S", about = format!("Syncanhronize leetcode info {}","[ alias: S ]".bold()))]
    Sync(Force),
    #[command(alias = "t", about = format!("Test your code {}", "[ alias: t ]".bold()))]
    Test(IdArg),
    #[command(alias = "st", about = format!("Submit your code {}", "[ alias: st ]".bold()))]
    Submit(IdArg),
    #[command(alias = "sl", about = format!("Get submit list {}", "[ alias: sl ]".bold()))]
    Sublist(IdArg),
    #[command(alias = "g", about = format!("Generate a config {}", "[ alias: g ]".bold()))]
    Gencon(GenArgs),
    #[command(alias = "T", about = format!("Open Tui {}", "[ alias: T ]".bold()))]
    Tui,
    #[command(alias = "C", about = format!("Edit config {}", "[ alias: C ]".bold()))]
    Config,
    #[command(alias = "L", about = format!("Open Log {}", "[ alias: L ]".bold()))]
    Log,
    #[command(about = format!("Give the project a star"))]
    Star,
}

#[derive(Debug)]
#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
struct GenArgs {
    #[arg(short, long, help = "Generate cn config")]
    cn: bool,
}

#[derive(Debug)]
#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
struct IdArg {
    #[arg(help = "Question id")]
    id: u32,
}

#[derive(Debug)]
#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
struct InterArgs {
    #[command(subcommand)]
    command: Option<DetailOrEdit>,
}

#[derive(Debug)]
#[derive(Subcommand)]
enum DetailOrEdit {
    #[command(about = "View detail(default)")]
    Detail(DetailArgsFzy),
    #[command(about = "Edit code")]
    Edit,
}
#[derive(Debug)]
#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
struct DetailArgsFzy {
    #[arg(short, long, help = "Force update question's information")]
    force: bool,
}

#[derive(Debug)]
#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
struct DetailArgs {
    #[arg(help = "Question id")]
    id:    u32,
    #[arg(short, long, help = "Force update question's information")]
    force: bool,
}

#[derive(Debug)]
#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
struct Force {
    #[arg(short, long, help = "Delete database for full re-sync")]
    force: bool,
}

#[derive(Debug)]
#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
struct EditArgs {
    #[command(subcommand)]
    command: Option<CoT>,

    #[command(flatten, help = "Id  of the be edited question")]
    id: Option<IdArg>,
}

#[derive(Debug)]
#[derive(Subcommand)]
enum CoT {
    #[command(about = "Edit code(default)")]
    Code(IdArg),
    #[command(about = "Edit test case")]
    Test(IdArg),
}

/// Cli runner
pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    if let Some(shell) = cli.generator {
        let mut cmd = Cli::command();
        print_completions(shell, &mut cmd);
        return Ok(());
    }
    else if let Some(cmd) = cli.command {
        match cmd {
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
                    CoT::Code(id) => Editor::open(IdSlug::Id(id.id), CodeTestFile::Code).await?,
                    CoT::Test(id) => Editor::open(IdSlug::Id(id.id), CodeTestFile::Test).await?,
                },
                None => {
                    if let Some(id) = args.id {
                        Editor::open(IdSlug::Id(id.id), CodeTestFile::Code).await?;
                    }
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

                        Editor::open(IdSlug::Id(id), CodeTestFile::Code).await?;
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
            Commands::Star => crate::star(),
            Commands::Tui => Box::pin(mytui::run()).await?,
            Commands::Config => Editor::edit_config()?,
            Commands::Log => Editor::edit_log()?,
        };
    }

    Ok(())
}
