use crate::resp::Value;

pub fn ping() -> Value {
    return Value::SimpleString("PONG".to_string());
}
