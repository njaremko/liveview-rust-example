use askama::Template;
use actix_web::{HttpRequest, HttpResponse, Responder};
use crate::Result;

#[derive(Template)]
#[template(path = "base.html", escape = "none")]
pub struct BaseTemplate {
    pub title: String,
    pub content: String,
}

#[derive(Template)]
#[template(path = "hello.html")]
pub struct ExTemplate {
    pub name: String,
    pub count: i32,
}

pub(crate) fn render(req: HttpRequest) -> Result<HttpResponse> {
    let hello = BaseTemplate {
        title: "Example".into(),
        content: ExTemplate {
            name: "fish".into(),
            count: 0,
        }.render()?,
    };

    Ok(HttpResponse::Ok()
        .body(hello.render()?))
}