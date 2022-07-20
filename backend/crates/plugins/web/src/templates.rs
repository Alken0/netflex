use axum::response::Html;
use tera::{Context, Tera};

#[derive(Clone)]
pub struct Templates(Tera);

impl Templates {
    pub fn new(templates: &str) -> Self {
        Self(Tera::new(templates).unwrap())
    }

    pub fn parse(&self, template: &str, content: &Context) -> Html<String> {
        let rendered = self.0.render(template, content).unwrap();
        Html::from(rendered)
    }
}
