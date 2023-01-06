use anyhow::Result;
use async_trait::async_trait;

pub mod list;
pub mod toggle;

#[async_trait]
pub trait Command<T> {
    async fn execute(&self, args: &T) -> Result<()>;
}
