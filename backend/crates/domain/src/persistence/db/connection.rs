use super::Transaction;
use crate::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Connection {
    type Transaction: Transaction;
    async fn transaction(&self) -> Result<Self::Transaction>;
}
