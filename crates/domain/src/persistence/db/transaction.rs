use crate::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Transaction {
    async fn commit(self) -> Result<()>;
}
