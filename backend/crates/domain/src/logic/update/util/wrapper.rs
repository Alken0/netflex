use crate::{error::Result, persistence::repos::file::FileInsert};
use std::{
    collections::hash_map::DefaultHasher,
    fs::Metadata,
    hash::{Hash, Hasher},
};
use tokio::fs::DirEntry;

pub struct DirEntryWrapper {
    path: Path,
    metadata: Metadata,
}

impl DirEntryWrapper {
    pub fn is_file(&self) -> bool {
        self.metadata.is_file()
    }

    pub fn path(self) -> String {
        self.path.into()
    }

    pub async fn wrap_entry(entry: DirEntry) -> Result<DirEntryWrapper> {
        let metadata = entry.metadata().await?;
        let path: Path = (&entry).into();
        return Ok(DirEntryWrapper { path, metadata });
    }
}

impl From<DirEntryWrapper> for FileInsert {
    fn from(e: DirEntryWrapper) -> Self {
        Self {
            hash: hash(&e.path.0),
            name: e.path.file_name().into(),
            mime: e.path.mime().into(),
            size: e.metadata.len(),
            path: e.path.into(),
        }
    }
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

struct Name(String);

impl From<Name> for String {
    fn from(e: Name) -> Self {
        e.0
    }
}

struct Mime(String);

impl From<Mime> for String {
    fn from(e: Mime) -> Self {
        e.0
    }
}

struct Path(String);
impl Path {
    ///TODO file_name never fails because of metadata call before but should return Result!
    fn file_name(&self) -> Name {
        let name = std::path::Path::new(&self.0)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        return Name(name);
    }

    fn mime(&self) -> Mime {
        if self.0.ends_with(".m3u8") {
            return Mime("application/x-mpegURL".to_string());
        }
        return match mime_guess::from_path(&self.0).first() {
            Some(e) => Mime(e.to_string()),
            None => Mime("".to_string()),
        };
    }
}

impl From<&DirEntry> for Path {
    fn from(e: &DirEntry) -> Self {
        Path(
            e.path()
                .to_string_lossy()
                .to_string()
                .replace("\\", "/")
                .replace("./", ""),
        )
    }
}

impl From<Path> for String {
    fn from(e: Path) -> Self {
        e.0
    }
}
