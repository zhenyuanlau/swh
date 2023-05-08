use super::CommandHandler;
use crate::cli::list::EnvStates::ENABLED;
use crate::core::config_file::ConfigFile;
use crate::util;
use crate::util::green_cell;
use async_trait::async_trait;
use clap::{Args, ValueEnum};
use miette::Result;

pub struct ListCommand;

#[derive(Args, Debug)]
pub struct ListArgs {
    pub state: Option<EnvStates>,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum EnvStates {
    ENABLED,
    DISABLED,
}

#[async_trait]
impl CommandHandler<ListArgs> for ListCommand {
    async fn process(&self, args: &ListArgs) -> Result<()> {
        let config = ConfigFile::load()?;
        let enabled = args.state.as_ref().map(|s| *s == ENABLED);
        let envs = config.get_envs(&enabled);
        if !envs.is_empty() {
            let header = vec![green_cell("#"), green_cell("env"), green_cell("enabled")];
            let table = util::table(header, envs, false);
            println!("{table}");
        }
        Ok(())
    }
}
