#[derive(Clone, Debug)]
pub enum Response {
    NullBulkString(),
    NullArray(),
    SimpleString(String),
    BulkString(String),
    Array(Vec<Response>),
}

impl Response {
    // https://redis.io/docs/latest/develop/reference/protocol-spec/
    pub fn serialize(self) -> String {
        match self {
            Response::NullBulkString() => format!("$-1\r\n"),
            Response::NullArray() => format!("*-1\r\n"),
            Response::SimpleString(s) => format!("+{}\r\n", s),
            Response::BulkString(s) => format!("${}\r\n{}\r\n", s.len(), s),
            _ => panic!("Unsupported value for serialize!"),
        }
    }
}
