use crate::resp::Value;

// https://redis.io/docs/latest/commands/quit/
pub fn quit() -> Value {
    return Value::SimpleString("OK".to_string());
}
