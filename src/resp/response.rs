use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use super::parser::parse_message;

#[derive(Clone, Debug)]
pub enum Value {
    NullBulkString(),
    NullArray(),
    SimpleString(String),
    BulkString(String),
    Array(Vec<Value>),
    SimpleError(String),
}

impl Value {
    // https://redis.io/docs/latest/develop/reference/protocol-spec/
    pub fn serialize(self) -> String {
        match self {
            Value::NullBulkString() => format!("$-1\r\n"),
            Value::NullArray() => format!("*-1\r\n"),
            Value::SimpleString(s) => format!("+{}\r\n", s),
            Value::BulkString(s) => format!("${}\r\n{}\r\n", s.len(), s),
            Value::SimpleError(s) => format!("-ERR {}\r\n", s),
            _ => panic!("Unsupported value for serialize!"),
        }
    }
}

pub struct ResponseHandler {
    stream: TcpStream,
    buffer: BytesMut,
}

impl ResponseHandler {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(512),
        }
    }

    pub async fn read_value(&mut self) -> Result<Option<Value>, String> {
        match self.stream.read_buf(&mut self.buffer).await {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return Ok(None);
                }
                match parse_message(self.buffer.split()) {
                    Ok((val, _)) => {
                        return Ok(Some(val));
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            Err(err) => Err(format!("Failed to read from stream: {}", err)),
        }
    }

    pub async fn write_value(&mut self, response: Value) -> Result<(), String> {
        let _ = self.stream.write(response.serialize().as_bytes()).await;
        Ok(())
    }
}
