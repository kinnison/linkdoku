//! A single-page application service which is a fallback for everything else
//! which isn't handled by an API call etc.

use std::{collections::HashMap, ffi::OsStr};

use axum::{
    extract::Query,
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
};

use include_dir::{include_dir, Dir};
use tracing::info;

static SPA_FILES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../frontend/dist");

const MIMETYPES: &[(&str, &str)] = &[
    ("js", "text/javascript"),
    ("css", "text/css"),
    ("png", "image/png"),
    ("svg", "image/svg+xml"),
    ("html", "text/html"),
    ("txt", "text/plain"),
    ("wasm", "application/wasm"),
];

async fn serve_file(filename: &str) -> Response {
    if let Some(file) = SPA_FILES.get_file(filename) {
        // Okay, we have the file, let's handle returning this asset
        let file_ext = file.path().extension().unwrap_or_else(|| OsStr::new(""));
        let content_type = MIMETYPES
            .iter()
            .find(|(ext, _)| OsStr::new(ext) == file_ext)
            .map(|(_, ty)| *ty)
            .unwrap_or("application/octet-stream");
        info!("Serving {filename} as {content_type}");
        let response = Response::builder()
            .header("Content-Type", content_type)
            .header("Content-Length", format!("{}", file.contents().len()))
            .status(StatusCode::OK)
            .body(())
            .unwrap();
        (response, file.contents()).into_response()
    } else {
        (StatusCode::NOT_FOUND, "not found").into_response()
    }
}

async fn ssr_render(uri: Uri, query: HashMap<String, String>) -> Response {
    // Acquire index.html
    let all_html = SPA_FILES
        .get_file("index.html")
        .unwrap()
        .contents_utf8()
        .unwrap();
    let (pre_head, rest) = all_html.split_once("</head>").unwrap();
    let (pre_body, rest) = rest.split_once("</body>").unwrap();

    let yew_body = yew::ServerRenderer::<frontend::App>::new().render().await;

    let full_body = format!("{pre_head}</head>{pre_body}{yew_body}</body>{rest}");
    (
        Response::builder()
            .header("Content-Type", "text/html")
            .header("Content-Length", format!("{}", full_body.len()))
            .status(StatusCode::OK)
            .body(())
            .unwrap(),
        full_body,
    )
        .into_response()
}

pub async fn spa_handler(uri: Uri, Query(query): Query<HashMap<String, String>>) -> Response {
    // Basically the rule is, if the uri starts /assets/ then we serve content from SPA_FILES
    // Otherwise we're trying to SSR the index.html

    if let Some(filename) = uri.path().strip_prefix("/assets/") {
        serve_file(filename).await
    } else {
        ssr_render(uri, query).await
    }
}
