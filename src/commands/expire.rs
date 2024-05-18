use std::sync::Arc;

use tokio::time::Instant;

use crate::{
    args,
    database::{Database, SetOptions},
    resp::Value,
};

// https://redis.io/docs/latest/commands/expire/
pub async fn expire(args: Vec<String>, db: &Arc<Database>) -> Value {
    if args.len() < 2 {
        return Value::SimpleError(
            "ERR wrong number of arguments for 'expire' command".to_string(),
        );
    }

    let key_arg = match args.get(0) {
        Some(k) => k.to_string(),
        None => return Value::Integer(0),
    };

    let value_arg = match args.get(1) {
        Some(v) => match v.parse::<u64>() {
            Ok(val) => val,
            Err(_) => {
                return Value::SimpleError(
                    "ERR value is not an integer or out of range".to_string(),
                )
            }
        },
        None => return Value::Integer(0),
    };

    match db.get(key_arg).await {
        Some(key) => {
            let expires = key.expires;

            // set expiry only when the key has no expiry
            if args::some(&args, "nx") {
                if expires {
                    return Value::Integer(0);
                }
            }

            // set expiry only when the key has an existing expiry
            if args::some(&args, "xx") {
                if !expires {
                    return Value::Integer(0);
                }
            }

            let now = Instant::now();
            let expires_at = key.expires_at;
            let ttl = expires_at.duration_since(now);

            // set expiry only when the new expiry is greater than current one
            if args::some(&args, "gt") {
                if value_arg < ttl.as_secs() {
                    return Value::Integer(0);
                }
            }

            // set expiry only when the new expiry is less than current one
            if args::some(&args, "lt") {
                if value_arg > ttl.as_secs() {
                    return Value::Integer(0);
                }
            }

            let mut options = SetOptions::default();
            options.expiry_in_ms = value_arg * 1000;

            let _ = db.set(key.key, key.value, options).await;
            Value::Integer(1)
        }
        None => return Value::Integer(0),
    }
}
