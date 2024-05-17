use std::sync::Arc;

use crate::{
    database::{Database, SetOptions},
    resp::Value,
};

// https://redis.io/docs/latest/commands/delete/
pub async fn del(args: Vec<String>, db: &Arc<Database>) -> Value {
    if args.len() < 1 {
        return Value::SimpleError("ERR wrong number of arguments for 'del' command".to_string());
    }

    let mut total_deleted = 0;

    for arg in args {
        let _ = db.delete(arg).await;
        total_deleted += 1;
    }

    Value::Integer(total_deleted as i64)
}
