use super::wrapper;
use super::wrapper::DirEntryWrapper;
use crate::error::Result;
use futures::future::join_all;
use tokio::fs::read_dir;
use tokio::fs::DirEntry;

async fn element_for_path(path: &String) -> Result<Vec<DirEntry>> {
    let mut entries = read_dir(path).await?;
    let mut paths = Vec::new();
    while let Ok(Some(entry)) = entries.next_entry().await {
        paths.push(entry);
    }
    return Ok(paths);
}

pub async fn elements(paths: &[String]) -> Vec<DirEntry> {
    let async_extract_elements = paths.iter().map(element_for_path);

    return join_all(async_extract_elements)
        .await
        .into_iter()
        .filter_map(|f| f.ok())
        .flatten()
        .collect();
}

pub async fn wrap_types(entries: Vec<DirEntry>) -> Vec<DirEntryWrapper> {
    let async_to_type = entries
        .into_iter()
        .map(wrapper::DirEntryWrapper::wrap_entry);

    return join_all(async_to_type)
        .await
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();
}

pub fn partition_by_type(entries: Vec<DirEntryWrapper>) -> DirContent {
    let (files, dirs) = entries.into_iter().partition(|e| e.is_file());
    return DirContent { files, dirs };
}

pub struct DirContent {
    pub files: Vec<DirEntryWrapper>,
    pub dirs: Vec<DirEntryWrapper>,
}

pub async fn extract(dirs: &[String]) -> DirContent {
    let elements = elements(dirs).await;
    let elements = wrap_types(elements).await;
    return partition_by_type(elements);
}
