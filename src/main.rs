use clap::Parser;
use log::error;

use miette::Result;
use swh::cli::list::ListCommand;
use swh::cli::open::OpenCommand;
use swh::cli::serve::ServeCommand;
use swh::cli::show::ShowCommand;
use swh::cli::toggle::ToggleCommand;
use swh::cli::{Cli, CommandHandler, Commands};
use swh::core::config_file::ConfigFile;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    ConfigFile::create_if_not_exists()?;

    let cli = Cli::parse();

    let future_result = match cli.command {
        Commands::List(ref args) => ListCommand.process(args),
        Commands::Toggle(ref args) => ToggleCommand.process(args),
        Commands::Show(ref args) => ShowCommand.process(args),
        Commands::Open(ref args) => OpenCommand.process(args),
        Commands::Serve(ref args) => ServeCommand.process(args),
    };

    match future_result.await {
        Ok(()) => {}
        Err(err) => {
            error!("{}", err);
            std::process::exit(1)
        }
    }
    Ok(())
}
