// use Duration
use {std::time::Duration, tokio::task::JoinHandle};

async fn foo(index: u32) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("child-{} sleep 400ms", index);
        }
    })
}

#[tokio::main]
async fn main() {
    let mut tasks: Vec<JoinHandle<()>> = Vec::new();
    let h1 = foo(1).await;
    let h2 = foo(2).await;
    tasks.push(h1);
    tasks.push(h2);

    let mut count = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("main sleep 1ms");
        count += 1;
        if count > 1 {
            for t in tasks.iter_mut() {
                t.abort();
            }
        } else if count > 2 {
            break;
        }
    }
}
