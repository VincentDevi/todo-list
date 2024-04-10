use askama::Template;
use axum::{
    body::Body,
    extract::Path,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;

#[derive(Template)]
#[template(path = "app.html")]
struct Index {
    count: u32,
}

#[derive(Template)]
#[template(path = "count.html")]
struct Count {
    count: u32,
}
impl IntoResponse for Count {
    fn into_response(self) -> Response {
        Response::new(Body::new(self.render().unwrap()))
    }
}

impl IntoResponse for Index {
    fn into_response(self) -> axum::response::Response {
        Response::new(Body::new(self.render().unwrap()))
    }
}

pub async fn index() -> impl IntoResponse {
    let index = Index { count: 10 };
    (StatusCode::OK, index)
}

pub async fn increment_counter(Path(count): Path<u32>) -> impl IntoResponse {
    let count = Count { count: count + 1 };
    (StatusCode::OK, count)
}
