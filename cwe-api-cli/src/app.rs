//! Defines the `App`, containing runtime state.

use crate::cli::Cli;
use anyhow::Result;
use clap::Parser as _;
use cwe_api::Client;

/// Holds the central app state and API client.
#[derive(Debug)]
pub struct App {
    /// The CLI arguments passed by the user.
    pub cli: Cli,
    /// The OpenAPI client.
    pub client: Client,
}

impl App {
    /// Construct a new `App` instance.
    pub fn new() -> Result<Self> {
        let cli = Cli::parse();
        let client = Client::new(cwe_api::BASE_URL);
        Ok(App { cli, client })
    }
}
