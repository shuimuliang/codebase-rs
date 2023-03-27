async fn foo() {
    println!("foo");
}

// https://tokio.rs/tokio/tutorial/spawning
// Tasks in Tokio are very lightweight.
// Under the hood, they require only a single allocation and 64 bytes of memory.
// Applications should feel free to spawn thousands, if not millions of tasks.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(foo());
    println!("Hello, world!");
    Ok(())
}
