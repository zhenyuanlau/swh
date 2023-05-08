use super::CommandHandler;
use crate::core::config_file::ConfigFile;
use async_trait::async_trait;
use clap::Args;
use miette::Result;

pub struct OpenCommand;

#[derive(Debug, Args)]
pub struct OpenArgs {
    #[arg(value_name = "CONFIG")]
    config: Option<String>,
}

#[async_trait]
impl CommandHandler<OpenArgs> for OpenCommand {
    async fn process(&self, _args: &OpenArgs) -> Result<()> {
        ConfigFile::create_if_not_exists()?;
        std::process::Command::new("vi")
            .arg(ConfigFile::swh_config_file_path().unwrap())
            .spawn()
            .expect("Error: Failed to run editor")
            .wait()
            .expect("Error: Editor returned a non-zero status");
        Ok(())
    }
}
