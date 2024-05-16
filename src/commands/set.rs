use std::sync::Arc;

use crate::{
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
    let mut current_arg = 2;
    while let Some(arg) = args.get(current_arg) {
        match arg.to_lowercase().as_str() {
            "px" => {
                current_arg += 1;
                if let Some(expiry) = args.get(current_arg) {
                    if let Ok(expiry_in_ms) = expiry.parse::<u64>() {
                        options.expiry_in_ms = expiry_in_ms;
                    }
                }
            }
            _ => {}
        }
        current_arg += 1;
    }

    let _ = db.set(key.clone(), value.clone(), options).await;
    Value::SimpleString("OK".to_string())
}
