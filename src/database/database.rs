use std::fmt::Error;

use tokio::sync::Mutex;
use tokio::time::Instant;

use crate::database::data::Data;
use crate::database::internal::Internal;
use crate::database::options::SetOptions;

pub struct Database {
    store: Mutex<Internal>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(Internal::new()),
        }
    }

    pub async fn get(&self, key: String) -> Option<Data> {
        let mut store = self.store.lock().await;

        match store.data.get(&key) {
            Some(res) => {
                let now = Instant::now();
                if res.expires && res.expires_at < now {
                    store.data.remove(&key);
                    return None;
                }
                return Some(res.clone());
            }
            _ => {}
        }

        None
    }

    pub async fn set(
        &self,
        key: String,
        value: String,
        options: SetOptions,
    ) -> Result<Data, Error> {
        let val = Data::new(key.clone(), value.clone(), options);
        self.store
            .lock()
            .await
            .data
            .entry(key)
            .and_modify(|v| *v = val.clone())
            .or_insert(val.clone());
        Ok(val)
    }
}
