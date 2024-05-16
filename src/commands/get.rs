use std::sync::Arc;

use crate::{database::Database, resp::Value};

// https://redis.io/docs/latest/commands/get/
pub async fn get(args: Vec<String>, db: &Arc<Database>) -> Value {
    if let Some(arg) = args.first() {
        if let Some(res) = db.get(arg.to_string()).await {
            return Value::BulkString(res.value);
        }
        return Value::NullBulkString();
    }
    Value::SimpleError("ERR wrong number of arguments for 'get' command".to_string())
}
