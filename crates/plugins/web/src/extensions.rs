use crate::{config::ServerConfig, templates::Templates};
use axum::{Extension, Router};
use domain::{
    config::Config,
    logic::{
        file::{
            find_all::FindAllAction, find_all_audios::FindAllAudiosAction,
            find_all_videos::FindAllVideosAction, find_by_hash::FindByHashAction,
            find_by_id::FindByIdAction,
        },
        stream::StreamAction,
        update::UpdateAction,
    },
};
use persistence::Database;

pub fn setup(
    router: Router,
    db: &Database,
    config: &Config,
    server_config: &ServerConfig,
) -> Router {
    let db = || db.clone();
    let config = || config.clone();

    router
        .layer(Extension(UpdateAction::new(db(), config())))
        .layer(Extension(StreamAction::new(db())))
        .layer(Extension(FindByHashAction::new(db())))
        .layer(Extension(FindByIdAction::new(db())))
        .layer(Extension(FindAllAction::new(db())))
        .layer(Extension(FindAllVideosAction::new(db())))
        .layer(Extension(FindAllAudiosAction::new(db())))
        .layer(Extension(Templates::new(&server_config.templates)))
}
