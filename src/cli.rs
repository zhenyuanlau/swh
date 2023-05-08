use self::list::ListArgs;
use self::open::OpenArgs;
use self::serve::ServeArgs;
use self::show::ShowArgs;
use self::toggle::ToggleArgs;

use async_trait::async_trait;
use clap::{Parser, Subcommand};
use miette::Result;

pub mod list;
pub mod open;
pub mod serve;
pub mod show;
pub mod toggle;

#[derive(Debug, Parser)]
#[command(name = "swh")]
#[command(bin_name = "swh")]
#[command(about = "A switch hosts CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Show hosts environment list
    List(ListArgs),
    /// Toggle hosts environment, eg. dev
    Toggle(ToggleArgs),
    /// Show hosts of an environment
    Show(ShowArgs),
    /// Open config file
    Open(OpenArgs),
    /// Start or stop HTTP server for third party
    Serve(ServeArgs),
}

#[async_trait]
pub trait CommandHandler<T> {
    async fn process(&self, args: &T) -> Result<()>;
}
