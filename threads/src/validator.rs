use crate::exit::Exit;
use core::time;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};
use std::thread::{self, sleep, Builder, JoinHandle};
use time::Duration;

pub struct TransactionStatusService {
    thread_hdl: JoinHandle<()>,
}

impl TransactionStatusService {
    pub fn new(exit: &Arc<AtomicBool>) -> Self {
        let exit = exit.clone();
        let thread_hdl = Builder::new()
            .name("solTxStatusWrtr".to_string())
            .spawn(move || loop {
                if exit.load(Ordering::Relaxed) {
                    break;
                }
                sleep(Duration::from_secs(1));
                println!("transaction thread sleep 1 seconds");
            })
            .unwrap();
        Self { thread_hdl }
    }

    pub fn join(self) -> thread::Result<()> {
        self.thread_hdl.join()
    }
}

pub struct EntryService {
    thread_hdl: JoinHandle<()>,
}

impl EntryService {
    pub fn new(exit: &Arc<AtomicBool>) -> Self {
        let exit = exit.clone();
        let thread_hdl = Builder::new()
            .name("solEntry".to_string())
            .spawn(move || loop {
                if exit.load(Ordering::Relaxed) {
                    break;
                }
                sleep(Duration::from_secs(1));
                println!("entry thread sleep 1 seconds");
            })
            .unwrap();
        Self { thread_hdl }
    }

    pub fn join(self) -> thread::Result<()> {
        self.thread_hdl.join()
    }
}

#[derive(Default)]
pub struct Validator {
    validator_exit: Arc<RwLock<Exit>>,
    transaction_status_service: Option<TransactionStatusService>,
    entry_service: Option<EntryService>,
}

impl Validator {
    pub fn new() -> Self {
        let exit = Arc::new(AtomicBool::new(false));
        let transaction_status_service = Some(TransactionStatusService::new(&exit));
        let entry_service = Some(EntryService::new(&exit));
        let validator_exit = Arc::new(RwLock::new(Exit::default()));
        // validator_exit.write().unwrap()
        // .register_exit(Box::new(move || exit.store(true, Ordering::Relaxed)));

        Self {
            validator_exit,
            transaction_status_service,
            entry_service,
        }
    }

    pub fn exit(&mut self) {
        self.validator_exit.write().unwrap().exit();
    }

    pub fn close(mut self) {
        self.exit();
        self.join();
    }

    pub fn join(self) {
        if let Some(transaction_status_service) = self.transaction_status_service {
            transaction_status_service
                .join()
                .expect("transaction_status_service");
        }
        if let Some(entry_service) = self.entry_service {
            entry_service.join().expect("entry_service");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_exit() {
        let validator = Validator::new();
        // validator.join();
        validator.close();
    }
}
