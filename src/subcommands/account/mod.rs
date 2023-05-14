use anyhow::Result;
use clap::{Parser, Subcommand};

mod fetch;
use fetch::Fetch;

mod oz;
use oz::Oz;

#[derive(Debug, Parser)]
pub struct Account {
    #[clap(subcommand)]
    command: Subcommands,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
    #[clap(about = "Fetch account config from an already deployed account contract")]
    Fetch(Fetch),
    #[clap(about = "Create, deploy, and manage OpenZeppelin account contracts")]
    Oz(Oz),
}

impl Account {
    pub async fn run(self) -> Result<()> {
        match self.command {
            Subcommands::Fetch(cmd) => cmd.run().await,
            Subcommands::Oz(cmd) => cmd.run().await,
        }
    }
}
