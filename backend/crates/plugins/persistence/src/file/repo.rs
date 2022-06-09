use super::entity;
use crate::Transaction;
use async_trait::async_trait;
use domain::{
    error::{database, Result},
    persistence::repos::{
        file::{DeleteByPath, File, FileInsert, FindAllWhereMimeLikeAny},
        general::{DeleteAll, FindAll, FindById, InsertAll},
    },
    types::Id,
};
use sea_orm::{
    sea_query::SimpleExpr, ColumnTrait, Condition, EntityTrait, Order, QueryFilter, QueryOrder,
};

#[async_trait]
impl DeleteByPath for Transaction {
    async fn delete_by_path(&self, path: &str) -> Result<()> {
        entity::Entity::delete_many()
            .filter(entity::Column::Path.like(&format!("{path}%")))
            .exec(&self.0)
            .await
            .map(|_| ())
            .map_err(|e| database(e.to_string()))
    }
}

#[async_trait]
impl InsertAll<FileInsert> for Transaction {
    async fn insert_all(&self, inserts: Vec<FileInsert>) -> Result<()> {
        // SQLITE-Limitation: throws error "Execution Error: error returned from database: too many SQL variables" if there are too many inserts at once
        let mut inserts = inserts.into_iter().map(Into::into).peekable();

        while inserts.peek().is_some() {
            let chunk: Vec<entity::ActiveModel> = inserts.by_ref().take(1024).collect();
            entity::Entity::insert_many(chunk)
                .exec(&self.0)
                .await
                .map(|_e| ())
                .map_err(|e| database(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl FindAll<File> for Transaction {
    async fn find_all(&self) -> Result<Vec<File>> {
        entity::Entity::find()
            .order_by(entity::Column::Id, Order::Asc)
            .all(&self.0)
            .await
            .map(|f| f.into_iter().map(File::from).collect())
            .map_err(|e| database(e.to_string()))
    }
}

#[async_trait]
impl FindAllWhereMimeLikeAny for Transaction {
    async fn find_all_where_mime_like_any(&self, mimes: &[&str]) -> Result<Vec<File>> {
        entity::Entity::find()
            .filter(mime_like_any(mimes))
            .order_by(entity::Column::Id, Order::Asc)
            .all(&self.0)
            .await
            .map(|f| f.into_iter().map(File::from).collect())
            .map_err(|e| database(e.to_string()))
    }
}

fn mime_like_any(mimes: &[&str]) -> SimpleExpr {
    if mimes.is_empty() {
        return entity::Column::Mime.not_like("*");
    }

    let mut expr = entity::Column::Mime.like(mimes[0]);
    let mimes_without_first_elment = &mimes[1..mimes.len()];
    for m in mimes_without_first_elment {
        expr = expr.or(entity::Column::Mime.like(m));
    }

    return expr;
}

#[async_trait]
impl DeleteAll<File> for Transaction {
    async fn delete_all(&self) -> Result<()> {
        entity::Entity::delete_many()
            .filter(
                Condition::any()
                    .add(entity::Column::Id.like("1"))
                    .add(entity::Column::Id.not_like("1")),
            )
            .exec(&self.0)
            .await
            .map(|_| ())
            .map_err(|e| database(e.to_string()))
    }
}

#[async_trait]
impl FindById<File> for Transaction {
    async fn find_by_id(&self, id: Id) -> Result<Option<File>> {
        entity::Entity::find_by_id(id.try_into().unwrap())
            .one(&self.0)
            .await
            .map(|e| e.map(Into::into))
            .map_err(|e| database(e.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod mime_like {
        use super::{entity, mime_like_any};
        use sea_orm::{sea_query::PostgresQueryBuilder, EntityTrait, QueryFilter};

        #[test]
        fn one_mime() {
            let mimes = vec!["first"];
            let expected = r#""file"."mime" LIKE 'first'"#;
            let result = postgres_query(&mimes);
            assert_eq!(result, expected);
        }

        #[test]
        fn two_mimes() {
            let mimes = vec!["first", "second"];
            let expected = r#"("file"."mime" LIKE 'first') OR ("file"."mime" LIKE 'second')"#;
            let result = postgres_query(&mimes);
            assert_eq!(result, expected);
        }

        #[test]
        fn three_mimes() {
            let mimes = vec!["first", "second", "third"];
            let expected = r#"("file"."mime" LIKE 'first') OR ("file"."mime" LIKE 'second') OR ("file"."mime" LIKE 'third')"#;
            let result = postgres_query(&mimes);
            assert_eq!(result, expected);
        }

        fn postgres_query(mimes: &[&str]) -> String {
            let redundant = entity::Entity::find()
                .query()
                .to_string(PostgresQueryBuilder);

            let query = entity::Entity::find()
                .filter(mime_like_any(&mimes))
                .query()
                .to_string(PostgresQueryBuilder);

            return query.replace(&format!("{redundant} WHERE "), "");
        }
    }
}
