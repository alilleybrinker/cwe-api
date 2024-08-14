use crate::cli::Cli;
use anyhow::Result;
use clap::Parser as _;
use cwe_api::Client;

const BASE_URL: &str = "https://cwe-api.mitre.org/api/v1";

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
        let client = {
            let client = reqwest::Client::builder().build()?;
            Client::new_with_client(BASE_URL, client)
        };

        Ok(App { cli, client })
    }
}
