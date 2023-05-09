use std::{collections::HashMap, future::Future, str::FromStr};

use axum::{
    debug_handler,
    extract::rejection::JsonRejection,
    http::{HeaderMap, HeaderValue, Uri},
    response::IntoResponse,
    routing::any,
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use serde_json::{json, Value};
use tokio::sync::mpsc::Sender;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", any(mirror))
        .route("/*anything", any(mirror));

    let port: u16 = value_from_env("SERVER_PORT").unwrap_or(80);
    let port_https: u16 = value_from_env("SERVER_PORT_HTTPS").unwrap_or(443);

    let config =
        RustlsConfig::from_pem_file("self-signed-certs/cert.pem", "self-signed-certs/key.pem")
            .await
            .unwrap();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(2);

    launch(
        axum_server::bind(format!("0.0.0.0:{port}").parse().unwrap())
            .serve(app.clone().into_make_service()),
        tx.clone(),
    );

    launch(
        axum_server::bind_rustls(format!("0.0.0.0:{port_https}").parse().unwrap(), config)
            .serve(app.into_make_service()),
        tx,
    );

    let _ = rx.recv().await;
}

fn value_from_env<T: FromStr>(env_var: &str) -> Option<T> {
    std::env::var(env_var)
        .into_iter()
        .flat_map(|v| v.parse())
        .next()
}

fn launch(fut: impl Future + Send + 'static, quit: Sender<()>) {
    tokio::task::spawn(async move {
        fut.await;
        quit.send(()).await.unwrap();
    });
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
