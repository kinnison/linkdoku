use std::{iter::Peekable, str::Chars, string::FromUtf16Error};

use serde_json::{Map, Value};

#[derive(Debug)]
pub enum ParseError {
    LZStrEncodingError,
    BadUnicode(FromUtf16Error),
    UnexpectEndOfInput,
    UnexpectedCharacter(char),
    JsonError(serde_json::Error),
}

pub type ParseResult<T> = std::result::Result<T, ParseError>;

pub fn parse_str(input: &str) -> ParseResult<Value> {
    let decoded = lz_str::decompress_from_base64(input).ok_or(ParseError::LZStrEncodingError)?;
    let decoded = String::from_utf16(&decoded).map_err(ParseError::BadUnicode)?;
    consume_object(&mut decoded.chars().peekable())
}

fn assert_next(seq: &mut Peekable<Chars>, ch: char) -> ParseResult<()> {
    match seq.next() {
        None => Err(ParseError::UnexpectEndOfInput),
        Some(n) => {
            if n == ch {
                Ok(())
            } else {
                Err(ParseError::UnexpectedCharacter(n))
            }
        }
    }
}

fn acquire_value(seq: &mut Peekable<Chars>) -> ParseResult<Value> {
    match seq.peek() {
        None => Err(ParseError::UnexpectEndOfInput),
        Some('{') => consume_object(seq),
        Some('[') => consume_array(seq),
        Some('\'') => consume_string(seq),
        Some('#') => consume_hexcode(seq),
        Some('t') => {
            seq.next();
            Ok(Value::Bool(true))
        }
        Some('f') => {
            seq.next();
            Ok(Value::Bool(false))
        }
        Some(n) if "0123456789".contains(*n) => consume_number(seq),
        Some(ch) => Err(ParseError::UnexpectedCharacter(*ch)),
    }
}

fn acquire_key(seq: &mut Peekable<Chars>) -> ParseResult<String> {
    let mut ret = String::new();
    loop {
        match seq.next() {
            None => return Err(ParseError::UnexpectEndOfInput),
            Some(':') => break,
            Some(ch) => ret.push(ch),
        }
    }
    Ok(ret)
}

fn consume_object(seq: &mut Peekable<Chars>) -> ParseResult<Value> {
    assert_next(seq, '{')?;
    let mut ret = serde_json::Map::new();

    loop {
        match seq.peek() {
            None => return Err(ParseError::UnexpectEndOfInput),
            Some('}') => break,
            _ => {
                // key:Value
                let key = acquire_key(seq)?;
                let value = acquire_value(seq)?;
                if matches!(seq.peek(), Some(',')) {
                    seq.next();
                }
                ret.insert(key, value);
            }
        }
    }
    assert_next(seq, '}')?;

    Ok(Value::Object(ret))
}

fn consume_array(seq: &mut Peekable<Chars>) -> ParseResult<Value> {
    assert_next(seq, '[')?;
    let mut ret = vec![];
    loop {
        match seq.peek() {
            None => return Err(ParseError::UnexpectEndOfInput),
            Some(']') => break,
            Some(',') => {
                seq.next();
                ret.push(Value::Object(Map::new()));
                if matches!(seq.peek(), Some(']')) {
                    ret.push(Value::Object(Map::new()));
                }
            }
            _ => {
                ret.push(acquire_value(seq)?);
                if matches!(seq.peek(), Some(',')) {
                    seq.next();
                }
            }
        }
    }
    assert_next(seq, ']')?;
    Ok(Value::Array(ret))
}

fn consume_string(seq: &mut Peekable<Chars>) -> ParseResult<Value> {
    assert_next(seq, '\'')?;
    let mut ret = String::new();
    let mut escaped = false;
    loop {
        let ch = seq.next().ok_or(ParseError::UnexpectEndOfInput)?;
        if escaped {
            match ch {
                'n' => ret.push('\n'),
                'r' => ret.push('\r'),
                _ => ret.push(ch),
            }
            escaped = false;
        } else {
            match ch {
                '\'' => break,
                '\\' => escaped = true,
                _ => ret.push(ch),
            }
        }
    }
    Ok(Value::String(ret))
}

fn consume_hexcode(seq: &mut Peekable<Chars>) -> ParseResult<Value> {
    assert_next(seq, '#')?;
    match seq.next() {
        None => Err(ParseError::UnexpectEndOfInput),
        Some('0') => Ok(Value::String("#000000".into())),
        Some('F') => Ok(Value::String("#FFFFFF".into())),
        Some(ch) => Err(ParseError::UnexpectedCharacter(ch)),
    }
}

fn consume_number(seq: &mut Peekable<Chars>) -> ParseResult<Value> {
    let mut s = String::new();
    loop {
        match seq.peek().cloned() {
            None => return Err(ParseError::UnexpectEndOfInput),
            Some(c) if "0123456789.".contains(c) => {
                seq.next();
                s.push(c);
            }
            _ => break,
        }
    }
    Ok(Value::Number(s.parse().map_err(ParseError::JsonError)?))
}
