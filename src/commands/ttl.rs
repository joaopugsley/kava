use std::sync::Arc;

use tokio::time::Instant;

use crate::{database::Database, resp::Value};

// https://redis.io/docs/latest/commands/ttl/
pub async fn ttl(args: Vec<String>, db: &Arc<Database>) -> Value {
    if let Some(arg) = args.first() {
        if let Some(res) = db.get(arg.to_string()).await {
            if !res.expires {
                return Value::Integer(-1); // key exists but does not expire
            }

            let now = Instant::now();
            let expires_at = res.expires_at;
            let ttl = expires_at.duration_since(now);

            return Value::Integer(ttl.as_secs() as i64);
        }
        return Value::Integer(-2); // key does not exist
    }
    Value::SimpleError("ERR wrong number of arguments for 'get' command".to_string())
}
