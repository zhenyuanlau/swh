use clap::{Parser, Subcommand};

use anyhow::Result;
use swh::cmd::list::{List, ListCommand};
use swh::cmd::toggle::{Toggle, ToggleCommand};
use swh::cmd::Command;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    List(List),
    Toggle(Toggle),
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    let future_result = match cli.command {
        Commands::List(ref args) => ListCommand.execute(args),
        Commands::Toggle(ref args) => ToggleCommand.execute(args),
    };
    match future_result.await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1)
        }
    }
    Ok(())
}
