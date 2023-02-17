use tokio;

async fn foo() {
    println!("foo");
}

// https://tokio.rs/tokio/tutorial/spawning
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(foo());
    println!("Hello, world!");
    Ok(())
}