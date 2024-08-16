//! CLI for interacting with the CWE API.

mod app;
mod cli;
mod log;
mod report;

use crate::{app::App, cli::*};
use anyhow::Result;
use function_name::named;
use std::process::ExitCode;
use tracing::error;

#[tokio::main]
async fn main() -> ExitCode {
    log::setup_logging();

    run()
        .await
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|err| {
            err.chain().for_each(|err| error!(err));
            ExitCode::FAILURE
        })
}

async fn run() -> Result<()> {
    let app = App::new()?;

    match &app.cli.command {
        Command::Version(args) => run_version_cmd(&app, args).await,
        Command::Cwe(args) => match &args.command {
            CweCommand::Version => run_cwe_version_cmd(&app).await,
            CweCommand::Info(args) => run_cwe_info_cmd(&app, args).await,
            CweCommand::Parents(args) => run_cwe_parents_cmd(&app, args).await,
            CweCommand::Descendants(args) => run_cwe_descendants_cmd(&app, args).await,
            CweCommand::Children(args) => run_cwe_children_cmd(&app, args).await,
            CweCommand::Ancestors(args) => run_cwe_ancestors_cmd(&app, args).await,
        },
        Command::Weakness(args) => match &args.command {
            WeaknessCommand::Info(args) => run_weakness_info_cmd(&app, args).await,
        },
        Command::View(args) => match &args.command {
            ViewCommand::Info(args) => run_view_info_cmd(&app, args).await,
        },
        Command::Category(args) => match &args.command {
            CategoryCommand::Info(args) => run_category_info_cmd(&app, args).await,
        },
    }
}

#[named]
async fn run_version_cmd(app: &App, args: &VersionArgs) -> Result<()> {
    report!(
        app,
        serde_json::json!({
            "cli_version": args.cli_version.then(|| version::version!()),
            "api_version": args.api_version.then(|| app.client.api_version())
        })
    )
}

#[named]
async fn run_cwe_version_cmd(app: &App) -> Result<()> {
    report!(app, app.client.get_cwe().await?.into_inner())
}

#[named]
async fn run_cwe_info_cmd(app: &App, args: &CweInfoArgs) -> Result<()> {
    report!(app, app.client.get_cwe_info(&args.id).await?.into_inner().0)
}

#[named]
async fn run_cwe_parents_cmd(app: &App, args: &CweParentsArgs) -> Result<()> {
    report!(
        app,
        app.client
            .get_cwe_parents(&args.id, args.view.as_deref())
            .await?
            .into_inner()
    )
}

#[named]
async fn run_cwe_descendants_cmd(app: &App, args: &CweDescendantsArgs) -> Result<()> {
    report!(
        app,
        app.client
            .get_cwe_descendants(&args.id, args.view.as_deref())
            .await?
            .into_inner()
    )
}

#[named]
async fn run_cwe_children_cmd(app: &App, args: &CweChildrenArgs) -> Result<()> {
    report!(
        app,
        app.client
            .get_cwe_children(&args.id, args.view.as_deref())
            .await?
            .into_inner()
    )
}

#[named]
async fn run_cwe_ancestors_cmd(app: &App, args: &CweAncestorsArgs) -> Result<()> {
    report!(
        app,
        app.client
            .get_cwe_ancestors(&args.id, args.primary, args.view.as_deref())
            .await?
            .into_inner()
    )
}

#[named]
async fn run_weakness_info_cmd(app: &App, args: &WeaknessInfoArgs) -> Result<()> {
    report!(
        app,
        app.client.get_cwe_weakness(&args.id).await?.into_inner()
    )
}

#[named]
async fn run_view_info_cmd(app: &App, args: &ViewInfoArgs) -> Result<()> {
    report!(app, app.client.get_cwe_view(&args.id).await?.into_inner())
}

#[named]
async fn run_category_info_cmd(app: &App, args: &CategoryInfoArgs) -> Result<()> {
    report!(
        app,
        app.client.get_cwe_category(&args.id).await?.into_inner()
    )
}
