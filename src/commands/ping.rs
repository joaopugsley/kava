use crate::resp::Value;

// https://redis.io/docs/latest/commands/ping/
pub fn ping(args: Vec<String>) -> Value {
    if args.len() > 0 {
        return Value::BulkString(format!("PONG \"{}\"", args.join(" ")));
    }
    return Value::SimpleString("PONG".to_string());
}
