use std::time::Duration;
use tokio::time::Instant;

use crate::database::options::SetOptions;

#[derive(Clone)]
pub struct Data {
    pub key: String,
    pub value: String,
    pub expires: bool,
    pub created_at: Instant,
    pub expires_at: Instant,
}

impl Data {
    pub fn new(key: String, value: String, options: SetOptions) -> Self {
        let now = Instant::now();
        let expires = options.expiry_in_ms > 0;
        let expire_date = now + Duration::from_millis(options.expiry_in_ms);
        Self {
            key,
            value,
            expires,
            created_at: now,
            expires_at: expire_date,
        }
    }
}
