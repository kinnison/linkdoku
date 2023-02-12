//! encoding CtC puzzles
//!

use serde_json::{Map, Value};

pub fn encode_str(puzzle: &Value) -> String {
    let mut enc = String::new();
    encode_value(&mut enc, puzzle);
    lz_str::compress_to_base64(enc.as_str())
}

fn encode_value(enc: &mut String, value: &Value) {
    match value {
        Value::Null => {}
        Value::Bool(b) => enc.push(if *b { 't' } else { 'f' }),
        Value::Number(n) => encode_rawstring(enc, &format!("{}", n)),
        Value::String(s) => encode_string(enc, s),
        Value::Array(arr) => encode_array(enc, arr),
        Value::Object(m) => encode_map(enc, m),
    }
}

fn encode_string(enc: &mut String, s: &str) {
    match s {
        "#000000" => enc.push_str("#0"),
        "#FFFFFF" => enc.push_str("#F"),
        _ => {
            enc.push('\'');
            encode_rawstring(enc, s);
            enc.push('\'');
        }
    }
}

fn encode_rawstring(enc: &mut String, s: &str) {
    for ch in s.chars() {
        match ch {
            '\'' => enc.push_str("\\'"),
            '\n' => enc.push_str("\\n"),
            _ => enc.push(ch),
        }
    }
}

fn encode_map(enc: &mut String, map: &Map<String, Value>) {
    enc.push('{');
    for (i, (k, v)) in map.iter().enumerate() {
        if i != 0 {
            enc.push(',');
        }
        enc.push_str(k);
        enc.push(':');
        encode_value(enc, v);
    }
    enc.push('}');
}

fn encode_array(enc: &mut String, arr: &[Value]) {
    enc.push('[');
    for (i, val) in arr.iter().enumerate() {
        if i != 0 {
            enc.push(',')
        }
        if val.as_object().map(|m| m.is_empty()) != Some(true) {
            encode_value(enc, val);
        }
    }
    enc.push(']');
}
