#![allow(dead_code)]

extern crate core;

pub mod atomic_demo;
pub mod exit;
pub mod validator;

use crossbeam_channel::bounded;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use std::{thread, time};

use std::sync::mpsc;

// 同步通道，阻塞线程
fn msp_sync() {
    let (tx, rx) = mpsc::sync_channel(0);

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(time::Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

// 异步通道，不阻塞线程
fn msp_async() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(time::Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

// All child threads that haven’t been manually joined will be automatically joined just before this
// function invocation ends.
fn scope_foo() {
    let (snd, rcv) = unbounded();
    let n_msgs = 5;

    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd.send(i).unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
        });
    })
    .unwrap();
    for _ in 0..n_msgs {
        let msg = rcv.recv().unwrap();
        println!("Received {}", msg);
    }

    foo();
}

// Sends the Fibonacci sequence into the channel until it becomes disconnected.
fn fibonacci(sender: Sender<u64>) {
    let (mut x, mut y) = (0, 1);
    while sender.send(x).is_ok() {
        let tmp = x;
        x = y;
        y += tmp;
    }
}

fn foo() {
    let (s, r) = bounded(0);
    thread::spawn(|| fibonacci(s));

    // Print the first 20 Fibonacci numbers.
    for num in r.iter().take(20) {
        println!("{}", num);
    }
}

fn arc_join() {
    use std::sync::Arc;
    use std::sync::Mutex;

    // 通过`Rc`实现`Mutex`的多所有权
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // 创建子线程，并将`Mutex`的所有权拷贝传入到子线程中
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    // 等待所有子线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 输出最终的计数结果
    println!("Result: {}", *counter.lock().unwrap());
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::{hint, thread};

    #[test]
    fn test_foo() {
        let spinlock = Arc::new(AtomicUsize::new(1));

        let spinlock_clone = Arc::clone(&spinlock);
        let thread = thread::spawn(move || {
            spinlock_clone.store(0, Ordering::SeqCst);
        });

        // Wait for the other thread to release the lock
        while spinlock.load(Ordering::SeqCst) != 0 {
            hint::spin_loop();
        }

        if let Err(panic) = thread.join() {
            println!("Thread had an error: {panic:?}");
        }
    }
}
