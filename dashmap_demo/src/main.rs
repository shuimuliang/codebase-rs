#![allow(dead_code)]

use dashmap::DashMap;

struct State {
    map: DashMap<u32, u32>,
}

const ARRAY_LENGTH: u32 = 100;

impl State {
    fn new() -> State {
        State {
            map: DashMap::new(),
        }
    }
    fn init_map(&self) {
        for i in 0..ARRAY_LENGTH {
            self.map.insert(i, i);
        }
    }
    fn _display(&self) {
        let mut res: Vec<(u32, u32)> = Vec::new();
        for i in 0..1 {
            if let Some(v) = self.map.get(&i) {
                res.push((*v.key(), *v.value()));
            }
        }
        println!("{:?}", res);
    }
    fn assert(&self, value: u32) {
        let i = 0;
        if let Some(v) = self.map.get(&i) {
            if *v.value() != value {
                assert!(false);
            }
        }
    }
}

#[tokio::main]
async fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_1() {
        let state = Arc::new(State::new());
        state.init_map();

        let state1 = state.clone();
        let state2 = state.clone();

        let h2 = tokio::spawn(async move {
            let i = 0;
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            if let Some(mut v) = state2.map.get_mut(&i) {
                *v = 2;
            }
        });

        let h1 = tokio::spawn(async move {
            let i = 0;
            tokio::time::sleep(std::time::Duration::from_millis(30)).await;
            if let Some(mut v) = state1.map.get_mut(&i) {
                *v = 1;
            }
        });

        h2.await.unwrap();
        h1.await.unwrap();
        state.assert(1);
    }

    #[tokio::test]
    async fn test_2() {
        let state = Arc::new(State::new());
        state.init_map();

        let state1 = state.clone();
        let state2 = state.clone();

        let h2 = tokio::spawn(async move {
            let i = 0;
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            if let Some(mut v) = state2.map.get_mut(&i) {
                *v = 2;
            }
        });

        let h1 = tokio::spawn(async move {
            let i = 0;
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            if let Some(mut v) = state1.map.get_mut(&i) {
                *v = 1;
            }
        });

        h2.await.unwrap();
        h1.await.unwrap();
        state.assert(2);
    }
}
