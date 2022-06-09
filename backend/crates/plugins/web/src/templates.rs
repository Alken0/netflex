use axum::response::Html;
use once_cell::sync::Lazy;
use tera::{Context, Tera};

static TERA: Lazy<Tera> =
    Lazy::new(|| Tera::new("crates/plugins/web/templates/**/*.html").unwrap());

pub fn parse(template: &str, content: &Context) -> Html<String> {
    let rendered = TERA.render(template, content).unwrap();
    Html::from(rendered)
}
