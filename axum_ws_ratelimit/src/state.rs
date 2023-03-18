use {
    dashmap::DashMap,
    std::{net::IpAddr, sync::Arc},
};

const WS_CONN_LIMIT: usize = 2;

#[derive(Clone, Debug)]
pub struct AppState {
    pub conn: DashMap<IpAddr, usize>,
}

pub type ShareState = Arc<AppState>;

impl AppState {
    pub fn new() -> AppState {
        AppState {
            conn: DashMap::new(),
        }
    }
    pub fn get_conn_count(&self, addr: &IpAddr) -> usize {
        match self.conn.get(addr) {
            Some(count) => *count,
            None => 0,
        }
    }
    pub fn add_conn(&self, addr: IpAddr) {
        *self.conn.entry(addr).or_insert(0) += 1;
    }
    pub fn remove_conn(&self, addr: IpAddr) {
        let res = self.conn.get_mut(&addr);
        if let Some(mut count) = res {
            if *count > 0 {
                *count -= 1;
            }
        }

        self.conn.remove_if(&addr, |_, count| *count == 0);
    }

    pub fn exist_conn(&self, addr: IpAddr) -> bool {
        self.conn.contains_key(&addr)
    }
    pub fn conn_exceed_limit(&self, addr: &IpAddr) -> bool {
        self.get_conn_count(addr) >= WS_CONN_LIMIT
    }
}

#[cfg(test)]
mod tests {
    use super::AppState;
    use std::net::IpAddr;

    #[test]
    fn test_state_add_conn() {
        let state = AppState::new();
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

    #[test]
    fn test_state_remove_conn() {
        let state = AppState::new();
        let addr = IpAddr::from([127, 0, 0, 1]);
        assert_eq!(state.exist_conn(addr), false);

        // count is 1, exist
        state.add_conn(addr);
        assert_eq!(state.exist_conn(addr), true);

        // count is 2, exist
        state.add_conn(addr);
        assert_eq!(state.exist_conn(addr), true);

        // count decrease to 1, exist
        state.remove_conn(addr);
        assert_eq!(state.exist_conn(addr), true);

        // count decrease to 0, not exist
        state.remove_conn(addr);
        assert_eq!(state.exist_conn(addr), false);
    }

    #[test]
    fn test_state_remove_conn_non_exist() {
        let state = AppState::new();
        let addr = IpAddr::from([127, 0, 0, 1]);
        assert_eq!(state.exist_conn(addr), false);

        state.remove_conn(addr);
        assert_eq!(state.exist_conn(addr), false);
    }
}
