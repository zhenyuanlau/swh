use crate::core::SWH_HTTP_API_URL;

use super::Command;
use anyhow::Result;
use async_trait::async_trait;
use clap::Args;

pub struct ToggleCommand;

#[derive(Debug, Args)]
pub struct Toggle {
    pub id: String,
}

#[async_trait]
impl Command<Toggle> for ToggleCommand {
    async fn execute(&self, args: &Toggle) -> Result<()> {
        let swh_api_toggle = format!("{}/api/toggle", SWH_HTTP_API_URL);
        let res = reqwest::Client::new()
            .get(swh_api_toggle)
            .query(&[("id", &args.id)])
            .send()
            .await?
            .text()
            .await
            .expect("Failed to call toggle api");
        println!("{}", res);
        Ok(())
    }
}
