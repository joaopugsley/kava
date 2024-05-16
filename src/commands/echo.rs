use crate::resp::Value;

// https://redis.io/docs/latest/commands/echo/
pub fn echo(args: Vec<String>) -> Value {
    if let Some(first_arg) = args.first() {
        return Value::BulkString(format!("\"{}\"", first_arg));
    }
    Value::SimpleError("ERR wrong number of arguments for 'echo' command".to_string())
}
