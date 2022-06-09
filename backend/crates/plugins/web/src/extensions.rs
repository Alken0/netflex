use axum::{Extension, Router};
use domain::logic::{
    file::{
        find_all::FindAllAction, find_all_audios::FindAllAudiosAction,
        find_all_videos::FindAllVideosAction, find_by_id::FindByIdAction,
    },
    stream::StreamAction,
    update::UpdateAction,
};
use persistence::Database;

pub fn setup(router: Router, db: &Database) -> Router {
    router
        .layer(Extension(UpdateAction::new(db.clone())))
        .layer(Extension(StreamAction::new(db.clone())))
        .layer(Extension(FindByIdAction::new(db.clone())))
        .layer(Extension(FindAllAction::new(db.clone())))
        .layer(Extension(FindAllVideosAction::new(db.clone())))
        .layer(Extension(FindAllAudiosAction::new(db.clone())))
}
