use askama::Template;
use axum::{
    body::Body,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

impl<'a> IntoResponse for HelloTemplate<'a> {
    fn into_response(self) -> axum::response::Response {
        Response::new(Body::new(self.render().unwrap()))
    }
}

pub async fn hello() -> impl IntoResponse {
    let test = HelloTemplate { name: "Vincent" };
    (StatusCode::OK, test)
}
