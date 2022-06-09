use super::entity;
use domain::persistence::repos::file::{File, FileInsert};
use sea_orm::{NotSet, Set};

impl From<FileInsert> for entity::ActiveModel {
    fn from(value: FileInsert) -> Self {
        Self {
            id: NotSet,
            name: Set(value.name),
            path: Set(value.path),
            mime: Set(value.mime),
            size: Set(value.size.try_into().unwrap()),
        }
    }
}

impl From<entity::Model> for File {
    fn from(value: entity::Model) -> Self {
        File {
            id: value.id.try_into().unwrap(),
            name: value.name,
            path: value.path,
            size: value.size.into(),
            mime: value.mime,
        }
    }
}
