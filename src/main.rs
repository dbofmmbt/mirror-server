use std::collections::HashMap;

use axum::{
    debug_handler,
    extract::rejection::JsonRejection,
    http::{HeaderMap, HeaderValue, Uri},
    response::IntoResponse,
    routing::any,
    Json, Router,
};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", any(mirror))
        .route("/*anything", any(mirror));

    let port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_owned())
        .parse()
        .unwrap();

    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn mirror(
    uri: Uri,
    headers: HeaderMap,
    body: Result<Json<Value>, JsonRejection>,
) -> impl IntoResponse {
    let response = json!({
        "server_name": std::env::var("SERVER_NAME").unwrap_or_else(|_| "name not set up".to_owned()),
        "request_path": uri.to_string(),
        "request_headers": convert(&headers),
        "request_body": *body.unwrap_or(Json(Value::Null)),
    });
    Json(response)
}

fn convert(headers: &HeaderMap<HeaderValue>) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for (k, v) in headers {
        let k = k.as_str().to_owned().replace('-', "_");
        let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
        map.entry(k).or_insert_with(Vec::new).push(v)
    }
    map
}
