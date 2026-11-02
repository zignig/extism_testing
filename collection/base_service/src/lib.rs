use wstd::http::error::Context;
use wstd::http::{Body, Request, Response, StatusCode};

#[wstd::http_server]
async fn main(req: Request<Body>) -> Result<Response<Body>, wstd::http::Error> {
    let host = req
        .uri()
        .authority()
        .map(|authority| authority.as_str())
        .unwrap_or("<none>");
    let body = format!("<H1>HELLO</H1>" );
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(body))
        .context("building response")
}
