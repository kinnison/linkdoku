//! FPuzzles functionality

use serde_json::Value;
use url::Url;

use crate::GridMetadata;

fn maybe_decode_lzstr(input: &str) -> Option<Value> {
    let decompressed = lz_str::decompress_from_base64(input)?;
    let decompressed = String::from_utf16(&decompressed).ok()?;
    serde_json::from_str(&decompressed).ok()
}

fn find_arg(url: &Url, key_to_find: &'static str) -> Option<String> {
    url.query_pairs()
        .find(|(key, _)| key == key_to_find)
        .map(|(_, value)| value)
        .map(|load| load.to_string())
}

pub fn extract<S: AsRef<str>>(input: S) -> Option<Value> {
    let input = input.as_ref();
    if let Ok(url) = Url::parse(input) {
        if let Some(host) = url.host_str() {
            // There's some kind of host, let's try and parse URL forms from here
            let host = host.to_ascii_lowercase();
            if let Some(data) = match host.as_str() {
                "f-puzzles.com" => find_arg(&url, "load"),
                _ if host.ends_with("sudokupad.app")
                    || host.ends_with("app.crackingthecryptic.com") =>
                {
                    find_arg(&url, "puzzleid")
                        .and_then(|s| s.strip_prefix("fpuzzles").map(String::from))
                        .or_else(|| {
                            url.query()
                                .and_then(|s| s.strip_prefix("fpuzzles").map(String::from))
                        })
                        .or_else(|| url.path().strip_prefix("/fpuzzles").map(String::from))
                }
                _ if host.ends_with("sudokulab.net") => find_arg(&url, "fpuzzle"),
                _ => None,
            } {
                // Unfortunately sometimes we end up with plusses in our encoded data, and that is needed
                // so reestablish those just in case
                let data = data.replace(' ', "+");
                if let Some(value) = maybe_decode_lzstr(&data) {
                    return Some(value);
                }
            }
        }
    }
    // If we get here we couldn't parse this usefully as fpuzzles data, try one last ditch effort
    maybe_decode_lzstr(input)
}

pub fn encode(input: &Value) -> String {
    let json_data = serde_json::to_string(input).expect("Odd, JSON encoding failed?");
    lz_str::compress_to_base64(json_data.as_str())
}

pub fn grid_url(input: &Value) -> String {
    format!(
        "https://api.sudokupad.com/thumbnail/fpuzzles{}_512x512.svg",
        encode(input)
    )
}

pub fn metadata(value: &Value) -> GridMetadata {
    let rows_cols = value
        .get("size")
        .and_then(Value::as_i64)
        .map(usize::try_from)
        .and_then(Result::ok)
        .map(|x| (x, x));
    let title = value.get("title").and_then(Value::as_str).map(String::from);
    let author = value
        .get("author")
        .and_then(Value::as_str)
        .map(String::from);
    let rules = value
        .get("ruleset")
        .and_then(Value::as_str)
        .map(String::from);
    let has_solution = value.get("solution").is_some();

    GridMetadata {
        title,
        author,
        rules,
        rows_cols,
        has_solution,
    }
}
