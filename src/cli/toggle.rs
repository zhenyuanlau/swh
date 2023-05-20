use super::CommandHandler;
use crate::core::config_file::ConfigFile;
use crate::core::host_file::HostFile;
use crate::util;
use async_trait::async_trait;
use clap::Args;
use miette::Result;

pub struct ToggleCommand;

#[derive(Debug, Args)]
pub struct ToggleArgs {
    pub env: String,
}

#[async_trait]
impl CommandHandler<ToggleArgs> for ToggleCommand {
    async fn process(&self, args: &ToggleArgs) -> Result<()> {
        util::escalate()?;
        let mut hf = HostFile::load()?;
        if let Ok(mut config) = ConfigFile::load() {
            match config.toggle(&args.env) {
                Ok(()) => {
                    ConfigFile::write(&config)?;
                    hf.sync(config)?;
                }
                Err(e) => {
                    log::info!("{}", e.to_string());
                }
            }
        }
        Ok(())
    }
}
