//! A single-page application service which is a fallback for everything else
//! which isn't handled by an API call etc.

use std::{collections::HashMap, ffi::OsStr};

use axum::{
    extract::{Path, Query, State},
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
};

use bounce::helmet::render_static;
use common::public::userinfo::UserInfo;
use database::Connection;
use include_dir::{include_dir, Dir};
use tokio::sync::Mutex;
use tower_cookies::Cookies;
use tracing::{info, warn};
use url::Url;

use crate::{
    config::ConfigState,
    login::{login_flow_status, PrivateCookies},
};

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

async fn find_linkdoku_svg() -> String {
    static SVG_CACHE: Mutex<Option<String>> = Mutex::const_new(None);

    let mut cache = SVG_CACHE.lock().await;

    if let Some(cached) = cache.as_ref() {
        return cached.clone();
    }

    for f in SPA_FILES.files() {
        let fname = f.path().file_name().unwrap().to_string_lossy();
        if fname.starts_with("linkdoku-") && fname.ends_with(".svg") {
            let ret = fname.to_string();
            *cache = Some(ret.clone());
            return ret;
        }
    }

    unreachable!()
}

#[tracing::instrument]
pub async fn serve_file(Path(filename): Path<String>) -> Response {
    info!("Serving {filename} from assets");
    if let Some(file) = SPA_FILES.get_file(&filename) {
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
            .header("Cache-Control", "public, max-age=2592000, immutable")
            .status(StatusCode::OK)
            .body(())
            .unwrap();
        (response, file.contents()).into_response()
    } else {
        (StatusCode::NOT_FOUND, "not found").into_response()
    }
}

#[tracing::instrument(skip(base, login, userinfo))]
async fn ssr_render(
    uri: Uri,
    query: HashMap<String, String>,
    base: &Url,
    login: Option<&str>,
    userinfo: Option<UserInfo>,
) -> Response {
    // Acquire index.html
    let all_html = SPA_FILES
        .get_file("index.html")
        .unwrap()
        .contents_utf8()
        .unwrap();
    let (pre_head, rest) = all_html.split_once("</head>").unwrap();
    let (pre_body, rest) = rest.split_once("</body>").unwrap();

    use frontend::ssr::*;

    info!("Performing SSR of {}", uri.to_string());

    let base = base.to_string();

    let login = login.map(str::to_string);

    let (header_renderer, header_writer) = render_static();

    let linkdoku_svg_asset = find_linkdoku_svg().await;

    let body = yew::ServerRenderer::<ServerApp>::with_props({
        let uri = uri.path().to_string();
        let base = base.clone();
        let span = sentry::configure_scope(|scope| {
            scope
                .get_span()
                .map(|span| span.start_child("ssr.render", &uri))
        });
        move || {
            let span = span.clone();
            sentry::configure_scope(|scope| {
                scope.set_span(span.clone().map(sentry::TransactionOrSpan::Span));
            });
            ServerAppProps {
                uri,
                query,
                base: base.into(),
                login,
                userinfo,
                header_writer,
                linkdoku_svg_asset: linkdoku_svg_asset.into(),
                span,
            }
        }
    })
    .render()
    .await;

    let rendered_header = header_renderer.render().await;

    let mut full_body = pre_head.to_string();
    full_body.push_str("\n    <base href=\"");
    full_body.push_str(&base);
    full_body.push_str("\" />\n");
    for t in rendered_header {
        t.write_static(&mut full_body).unwrap();
        full_body.push('\n');
    }
    full_body.push_str("    <!-- Page rendered due to request for ");
    full_body.push_str(&uri.to_string());
    full_body.push_str(" -->\n</head>");
    full_body.push_str(pre_body);
    full_body.push_str(&body);
    full_body.push_str("</body>");
    full_body.push_str(rest);

    (
        Response::builder()
            .header("Content-Type", "text/html")
            .header("Content-Length", format!("{}", full_body.len()))
            .header("Cache-Control", "no-store")
            .status(StatusCode::OK)
            .body(())
            .unwrap(),
        full_body,
    )
        .into_response()
}

pub async fn spa_handler(
    uri: Uri,
    Query(query): Query<HashMap<String, String>>,
    State(config): State<ConfigState>,
    cookies: Cookies,
    privatecookies: PrivateCookies,
    mut db: Connection,
) -> Response {
    // Basically the rule is, if the uri starts /assets/ then we serve content from SPA_FILES
    // Otherwise we're trying to SSR the index.html

    let login_cookie = cookies.get("login");
    let flow = login_flow_status(&privatecookies).await;
    let userinfo = match flow.user() {
        None => None,
        Some(user) => match user.identity().roles(&mut db).await {
            Ok(roles) => Some(UserInfo {
                uuid: user.identity().uuid.clone(),
                display_name: user.identity().display_name.clone(),
                gravatar_hash: user.identity().gravatar_hash.clone(),
                roles: roles.into_iter().map(|r| r.uuid).collect(),
                default_role: user.identity().default_role_uuid(),
            }),
            Err(e) => {
                warn!("Unable to read role data during SSR: {e:?}");
                None
            }
        },
    };
    ssr_render(
        uri,
        query,
        &config.base_url,
        login_cookie.as_ref().map(|cookie| cookie.value()),
        userinfo,
    )
    .await
}
