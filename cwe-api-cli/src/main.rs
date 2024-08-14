//! CLI for interacting with the CWE API.

mod app;
mod cli;

use crate::app::App;
use anyhow::{anyhow, Result};
use cli::{
    CategoryArgs, CategoryCommand, CategoryInfoArgs, Command, CweAncestorsArgs, CweArgs,
    CweChildrenArgs, CweCommand, CweDescendantsArgs, CweInfoArgs, CweParentsArgs, VersionArgs,
    ViewArgs, ViewCommand, ViewInfoArgs, WeaknessArgs, WeaknessCommand, WeaknessInfoArgs,
};
use serde::Serialize;
use std::process::ExitCode;
use tracing::instrument;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

#[tokio::main]
async fn main() -> ExitCode {
    setup_logging();

    match run().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            for err in err.chain() {
                eprintln!("error: {}", err);
            }

            ExitCode::FAILURE
        }
    }
}

fn setup_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
}

async fn run() -> Result<()> {
    let app = App::new()?;

    match &app.cli.command {
        Command::Version(args) => run_version_cmd(&app, args).await,
        Command::Cwe(args) => run_cwe_cmd(&app, args).await,
        Command::Weakness(args) => run_weakness_cmd(&app, args).await,
        Command::View(args) => run_view_cmd(&app, args).await,
        Command::Category(args) => run_category_cmd(&app, args).await,
    }
}

#[instrument]
async fn run_version_cmd(app: &App, args: &VersionArgs) -> Result<()> {
    let cli_version = args.cli_version.then(|| version::version!());
    let api_version = args.api_version.then(|| app.client.api_version());
    let std_version = match args.std_version {
        true => Some(
            app.client
                .get_cwe()
                .await?
                .into_inner()
                .content_version
                .ok_or_else(|| anyhow!("no version received from API"))?,
        ),
        false => None,
    };

    let value = serde_json::json!({
        "cli_version": cli_version,
        "api_version": api_version,
        "std_version": std_version
    });

    report_json(value)
}

#[instrument]
async fn run_cwe_cmd(app: &App, args: &CweArgs) -> Result<()> {
    match &args.command {
        CweCommand::Info(args) => run_cwe_info_cmd(app, args).await,
        CweCommand::Parents(args) => run_cwe_parents_cmd(app, args).await,
        CweCommand::Descendants(args) => run_cwe_descendants_cmd(app, args).await,
        CweCommand::Children(args) => run_cwe_children_cmd(app, args).await,
        CweCommand::Ancestors(args) => run_cwe_ancestors_cmd(app, args).await,
    }
}

async fn run_cwe_info_cmd(app: &App, args: &CweInfoArgs) -> Result<()> {
    let value = app.client.get_cwe_info(&args.id).await?.into_inner().0;
    report_json(value)
}

async fn run_cwe_parents_cmd(app: &App, args: &CweParentsArgs) -> Result<()> {
    let value = app
        .client
        .get_cwe_parents(&args.id, args.view.as_deref())
        .await?
        .into_inner();
    report_json(value)
}

async fn run_cwe_descendants_cmd(app: &App, args: &CweDescendantsArgs) -> Result<()> {
    let value = app
        .client
        .get_cwe_descendants(&args.id, args.view.as_deref())
        .await?
        .into_inner();
    report_json(value)
}

async fn run_cwe_children_cmd(app: &App, args: &CweChildrenArgs) -> Result<()> {
    let value = app
        .client
        .get_cwe_children(&args.id, args.view.as_deref())
        .await?
        .into_inner();
    report_json(value)
}

async fn run_cwe_ancestors_cmd(app: &App, args: &CweAncestorsArgs) -> Result<()> {
    let value = app
        .client
        .get_cwe_ancestors(&args.id, args.primary, args.view.as_deref())
        .await?
        .into_inner();
    report_json(value)
}

#[instrument]
async fn run_weakness_cmd(app: &App, args: &WeaknessArgs) -> Result<()> {
    match &args.command {
        WeaknessCommand::Info(args) => run_weakness_info_cmd(app, args).await,
    }
}

async fn run_weakness_info_cmd(app: &App, args: &WeaknessInfoArgs) -> Result<()> {
    let value = app.client.get_cwe_weakness(&args.id).await?.into_inner();
    report_json(value)
}

#[instrument]
async fn run_view_cmd(app: &App, args: &ViewArgs) -> Result<()> {
    match &args.command {
        ViewCommand::Info(args) => run_view_info_cmd(app, args).await,
    }
}

async fn run_view_info_cmd(app: &App, args: &ViewInfoArgs) -> Result<()> {
    let value = app.client.get_cwe_view(&args.id).await?.into_inner();
    report_json(value)
}

#[instrument]
async fn run_category_cmd(app: &App, args: &CategoryArgs) -> Result<()> {
    match &args.command {
        CategoryCommand::Info(args) => run_category_info_cmd(app, args).await,
    }
}

async fn run_category_info_cmd(app: &App, args: &CategoryInfoArgs) -> Result<()> {
    let value = app.client.get_cwe_category(&args.id).await?.into_inner();
    report_json(value)
}

fn report_json<T: Serialize>(value: T) -> Result<()> {
    let to_print = serde_json::to_string_pretty(&value)?;
    println!("{}", to_print);
    Ok(())
}
