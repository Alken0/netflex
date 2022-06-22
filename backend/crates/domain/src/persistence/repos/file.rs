use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FileInsert {
    pub name: String,
    pub size: u64,
    pub path: String,
    pub mime: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub id: u64,
    pub name: String,
    pub size: u64,
    pub path: String,
    pub mime: String,
}

#[async_trait]
pub trait FindByPath {
    async fn find_by_path(&self, path: &str) -> Result<Option<File>>;
}

#[async_trait]
pub trait DeleteByPath {
    async fn delete_by_path(&self, path: &str) -> Result<()>;
}

#[async_trait]
pub trait FindAllWhereMimeLikeAny {
    async fn find_all_where_mime_like_any(&self, mimes: &[&str]) -> Result<Vec<File>>;
}
