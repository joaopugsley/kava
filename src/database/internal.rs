use std::collections::HashMap;

use crate::database::data::Data;

pub struct Internal {
    pub data: HashMap<String, Data>,
}

impl Internal {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}
