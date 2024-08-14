#[derive(Debug, clap::Parser)]
#[clap(about, long_about=None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Get version information for the CLI and API.
    Version(VersionArgs),
    /// Operations for specific CWEs.
    Cwe(CweArgs),
    /// Operations for specific weaknesses.
    Weakness(WeaknessArgs),
    /// Operations for specific views.
    View(ViewArgs),
    /// Operations for specific categories.
    Category(CategoryArgs),
}

#[derive(Debug, clap::Args)]
pub struct VersionArgs {
    /// Get the version of the CLI binary.
    #[clap(long = "cli", default_value_t = true)]
    pub cli_version: bool,
    /// Get the version of the CWE API.
    #[clap(long = "api", default_value_t = false)]
    pub api_version: bool,
    /// Get the version of the CWE standard being used.
    #[clap(long = "std", default_value_t = false)]
    pub std_version: bool,
}

#[derive(Debug, clap::Args)]
pub struct CweArgs {
    #[clap(subcommand)]
    pub command: CweCommand,
}

#[derive(Debug, clap::Args)]
pub struct WeaknessArgs {
    #[clap(subcommand)]
    pub command: WeaknessCommand,
}

#[derive(Debug, clap::Args)]
pub struct ViewArgs {
    #[clap(subcommand)]
    pub command: ViewCommand,
}

#[derive(Debug, clap::Args)]
pub struct CategoryArgs {
    #[clap(subcommand)]
    pub command: CategoryCommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum CweCommand {
    /// Get info for a specific CWE.
    Info(CweInfoArgs),
    /// Get the parents of a specific CWE.
    Parents(CweParentsArgs),
    /// Get the descendants of a specific CWE.
    Descendants(CweDescendantsArgs),
    /// Get the children of a specific CWE.
    Children(CweChildrenArgs),
    /// Get the ancestors of a specific CWE.
    Ancestors(CweAncestorsArgs),
}

#[derive(Debug, clap::Args)]
pub struct CweInfoArgs {
    /// The ID of the CWE
    pub id: String,
}

#[derive(Debug, clap::Args)]
pub struct CweParentsArgs {
    /// The ID of the CWE
    pub id: String,
    /// An optional view filter
    #[clap(long)]
    pub view: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct CweDescendantsArgs {
    /// The ID of the CWE
    pub id: String,
    /// An optional view filter
    #[clap(long)]
    pub view: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct CweChildrenArgs {
    /// The ID of the CWE
    pub id: String,
    /// An optional view filter
    #[clap(long)]
    pub view: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct CweAncestorsArgs {
    /// The ID of the CWE
    pub id: String,
    /// The primary ancestor.
    #[clap(long)]
    pub primary: Option<bool>,
    /// An optional view filter
    #[clap(long)]
    pub view: Option<String>,
}

#[derive(Debug, clap::Subcommand)]
pub enum WeaknessCommand {
    /// Get info about a specific weakness.
    Info(WeaknessInfoArgs),
}

#[derive(Debug, clap::Args)]
pub struct WeaknessInfoArgs {
    /// The ID of the weakness.
    pub id: String,
}

#[derive(Debug, clap::Subcommand)]
pub enum ViewCommand {
    /// Get info about a specific view.
    Info(ViewInfoArgs),
}

#[derive(Debug, clap::Args)]
pub struct ViewInfoArgs {
    /// The ID of the view.
    pub id: String,
}

#[derive(Debug, clap::Subcommand)]
pub enum CategoryCommand {
    /// Get info about a specific category.
    Info(CategoryInfoArgs),
}

#[derive(Debug, clap::Args)]
pub struct CategoryInfoArgs {
    /// The ID of the category.
    pub id: String,
}
