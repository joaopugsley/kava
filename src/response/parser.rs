use std::num::ParseIntError;

use bytes::BytesMut;

use super::response::Response;

pub fn parse_message(buffer: BytesMut) -> Result<(Response, usize), String> {
    match buffer[0] as char {
        '+' => parse_simple_string(buffer),
        '$' => parse_bulk_string(buffer),
        _ => Err(format!("Not a known value type {:?}", buffer)),
    }
}

fn parse_simple_string(buffer: BytesMut) -> Result<(Response, usize), String> {
    if let Some((buf, len)) = read_until_crlf(&buffer[1..]) {
        let string = String::from_utf8(buf.to_vec()).unwrap();
        return Ok((Response::SimpleString(string), len + 1));
    }
    Err(format!(
        "Invalid simple string format: missing CRLF {:?}",
        buffer
    ))
}

fn parse_bulk_string(buffer: BytesMut) -> Result<(Response, usize), String> {
    let (bulk_str_length, bytes_consumed) = if let Some((buf, len)) = read_until_crlf(&buffer[1..])
    {
        if let Ok(bulk_str_length) = parse_int(buf) {
            (bulk_str_length, len + 1)
        } else {
            return Err(format!("Failed to parse bulk string length: {:?}", buffer));
        }
    } else {
        return Err(format!(
            "Invalid bulk string format: missing CRLF {:?}",
            buffer
        ));
    };

    let end_of_bulk_str = bytes_consumed + bulk_str_length as usize;
    let total_parsed = end_of_bulk_str + 2;

    if end_of_bulk_str > buffer.len() {
        return Err(format!(
            "Invalid bulk string format: length exceeds buffer size: {:?}",
            buffer
        ));
    }

    if let Ok(string) = String::from_utf8(buffer[bytes_consumed..end_of_bulk_str].to_vec()) {
        return Ok((Response::BulkString(string), total_parsed));
    }

    Err(format!(
        "Failed to convert bulk string to UTF-8: {:?}",
        buffer
    ))
}

fn read_until_crlf(buffer: &[u8]) -> Option<(&[u8], usize)> {
    for i in 1..buffer.len() {
        if buffer[i - 1] == b'\r' && buffer[i] == b'\n' {
            return Some((&buffer[0..(i - 1)], i + 1));
        }
    }
    None
}

fn parse_int(buffer: &[u8]) -> Result<i64, ParseIntError> {
    String::from_utf8(buffer.to_vec()).unwrap().parse::<i64>()
}
