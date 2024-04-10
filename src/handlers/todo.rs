use askama::Template;
use axum::{
    body::Body,
    extract::Path,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;

#[derive(Template)]
#[template(path = "count.html")]
struct Counter {
    count: u32,
}

impl IntoResponse for Counter {
    fn into_response(self) -> axum::response::Response {
        Response::new(Body::new(self.render().unwrap()))
    }
}

pub async fn increment_counter(Path(count): Path<u32>) -> impl IntoResponse {
    let count = Counter { count: count + 1 };
    (StatusCode::OK, count)
}
