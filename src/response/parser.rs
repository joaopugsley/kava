use std::num::ParseIntError;

use bytes::BytesMut;

use super::response::Response;

pub fn parse_message(buffer: BytesMut) -> Result<(Response, usize), String> {
    match buffer[0] as char {
        '+' => parse_simple_string(buffer),
        _ => Err(format!("Not a known value type {:?}", buffer)),
    }
}

fn parse_simple_string(buffer: BytesMut) -> Result<(Response, usize), String> {
    if let Some((buf, len)) = read_until_crlf(&buffer[1..]) {
        let string = String::from_utf8(buf.to_vec()).unwrap();
        return Ok((Response::SimpleString(string), len + 1));
    }
    Err(format!("Invalid string {:?}", buffer))
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
