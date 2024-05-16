pub struct SetOptions {
    pub expiry_in_ms: u64,
}

impl SetOptions {
    pub fn default() -> Self {
        Self { expiry_in_ms: 0 }
    }
}
