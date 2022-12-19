use crate::error::Result;
use crate::types::Id;
use async_trait::async_trait;

#[async_trait]
pub trait FindById<Entity> {
    async fn find_by_id(&self, id: Id) -> Result<Option<Entity>>;
}

#[async_trait]
pub trait FindAll<Entity> {
    async fn find_all(&self) -> Result<Vec<Entity>>;
}

#[async_trait]
pub trait InsertAll<Insert> {
    async fn insert_all(&self, inserts: Vec<Insert>) -> Result<()>;
}

#[async_trait]
pub trait DeleteAll<Entity> {
    async fn delete_all(&self) -> Result<()>;
}
