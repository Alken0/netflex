use super::entity;
use domain::persistence::repos::file::{File, FileInsert};
use numbers::ConverterU64U32;
use sea_orm::{NotSet, Set};

impl From<FileInsert> for entity::ActiveModel {
    fn from(value: FileInsert) -> Self {
        let hash = ConverterU64U32::new_by_u64(value.hash);
        let size = ConverterU64U32::new_by_u64(value.size);
        Self {
            id: NotSet,
            hash_high: Set(hash.high()),
            hash_low: Set(hash.low()),
            name: Set(value.name),
            path: Set(value.path),
            mime: Set(value.mime),
            size_high: Set(size.high()),
            size_low: Set(size.low()),
        }
    }
}

impl From<entity::Model> for File {
    fn from(value: entity::Model) -> Self {
        let hash = ConverterU64U32::new_by_u32(value.hash_high, value.hash_low);
        let size = ConverterU64U32::new_by_u32(value.size_high, value.size_low);
        File {
            id: value.id.try_into().unwrap(),
            name: value.name,
            path: value.path,
            size: size.total(),
            mime: value.mime,
            hash: hash.total(),
        }
    }
}
