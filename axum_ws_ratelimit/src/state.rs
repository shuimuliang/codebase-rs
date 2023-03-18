use std::{
    collections::HashMap,
    net::IpAddr,
    sync::{Arc, RwLock},
};

const WS_CONN_LIMIT: usize = 2;

#[derive(Clone, Debug)]
pub struct AppState {
    pub conn: HashMap<IpAddr, usize>,
}

pub type ShareState = Arc<RwLock<AppState>>;

impl AppState {
    pub fn new() -> AppState {
        AppState {
            conn: HashMap::new(),
        }
    }
    pub fn get_conn_count(&self, addr: &IpAddr) -> usize {
        match self.conn.get(addr) {
            Some(count) => *count,
            None => 0,
        }
    }
    pub fn add_conn(&mut self, addr: IpAddr) {
        *self.conn.entry(addr).or_insert(0) += 1;
    }
    pub fn remove_conn(&mut self, addr: IpAddr) {
        *self.conn.entry(addr).or_insert(0) -= 1;
    }
    pub fn conn_exceed_limit(&self, addr: &IpAddr) -> bool {
        self.get_conn_count(addr) > WS_CONN_LIMIT
    }
}

#[cfg(test)]
mod tests {
    use super::AppState;
    use std::net::IpAddr;

    #[test]
    fn test_state_add_conn() {
        let mut state = AppState::new();
        let addr1 = IpAddr::from([127, 0, 0, 1]);
        // ::1
        let addr2 = IpAddr::from([0, 0, 0, 0, 0, 0, 0, 1]);
        let addr3 = IpAddr::from([10, 0, 0, 1]);
        assert_eq!(state.get_conn_count(&addr1), 0);
        assert_eq!(state.get_conn_count(&addr2), 0);
        assert_eq!(state.get_conn_count(&addr3), 0);

        assert!(!state.conn_exceed_limit(&addr1));
        assert!(!state.conn_exceed_limit(&addr2));
        assert!(!state.conn_exceed_limit(&addr3));

        state.add_conn(addr1);
        assert_eq!(state.get_conn_count(&addr1), 1);
        state.add_conn(addr1);
        assert_eq!(state.get_conn_count(&addr1), 2);

        state.add_conn(addr1);
        assert!(state.conn_exceed_limit(&addr1));
    }
}
