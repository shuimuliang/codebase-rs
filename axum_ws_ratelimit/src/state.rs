use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub client_map: HashMap<u32, u32>,
}

pub type ShareState = Arc<RwLock<AppState>>;

impl AppState {
    pub fn new() -> AppState {
        AppState {
            client_map: HashMap::new(),
        }
    }
}
