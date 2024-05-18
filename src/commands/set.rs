use std::sync::Arc;

use crate::{
    args,
    database::{Database, SetOptions},
    resp::Value,
};

// https://redis.io/docs/latest/commands/set/
pub async fn set(args: Vec<String>, db: &Arc<Database>) -> Value {
    let key = match args.get(0) {
        Some(k) => k,
        None => {
            return Value::SimpleError(
                "ERR wrong number of arguments for 'set' command".to_string(),
            )
        }
    };

    let default_value = "".to_string();
    let value = args.get(1).unwrap_or(&default_value);

    let mut options = SetOptions::default();

    if let Some(px) = args::get::<u64>(&args, "px") {
        options.expiry_in_ms = px;
    }

    let _ = db.set(key.clone(), value.clone(), options).await;
    Value::SimpleString("OK".to_string())
}
