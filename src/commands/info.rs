use crate::resp::Value;

// https://redis.io/docs/latest/commands/info/
pub fn info() -> Value {
    return Value::SimpleString(format!(
        "# KAVA\n# SERVER\nKAVA_VERSION: {}",
        env!("CARGO_PKG_VERSION")
    ));
}
