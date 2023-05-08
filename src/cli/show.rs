use super::CommandHandler;
use crate::core::config_file::ConfigFile;
use crate::util;
use async_trait::async_trait;
use clap::Args;
use miette::Result;

pub struct ShowCommand;

#[derive(Args, Debug)]
pub struct ShowArgs {
    pub env: Option<String>,
}

#[async_trait]
impl CommandHandler<ShowArgs> for ShowCommand {
    async fn process(&self, args: &ShowArgs) -> Result<()> {
        let config = ConfigFile::load().unwrap();
        let hosts = config.show(args.env.as_ref());
        if !hosts.is_empty() {
            let colored = match args.env.as_ref() {
                Some(n) => config.get_env(n.as_str()).enabled,
                None => true,
            };
            let table = util::table(vec![], hosts, colored);
            println!("{table}");
        }
        Ok(())
    }
}
