#![allow(dead_code, unused_imports)]

use solana_blockstore::blockstore;
use solana_blockstore::blockstore::PurgeType;

#[derive(Default, Debug)]
struct EntryServices {
    entry_sender: Option<i32>,
    entry_service: Option<i32>,
}

impl EntryServices {
    fn foo1() {
        println!("foo1");
    }
}

impl EntryServices {
    fn foo2() {
        println!("foo1");
    }
}

fn case_1() {
    let t = EntryServices::default();
    println!("{:?}", t);
    EntryServices::foo1();
    EntryServices::foo2();
}

fn case_2() {
    PurgeType::foo();
}

fn main() {
    case_2();
}
