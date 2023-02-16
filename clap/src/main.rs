use clap::Parser;
use std::collections::HashMap;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(long)]
    x_token: Option<String>,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

type SMap = HashMap<String, String>;

#[derive(Debug)]
struct Foo {
    #[allow(dead_code)]
    a: SMap,
}

fn foo(a: &SMap) {
    let v = Foo { a: a.clone() };
    println!("{:?}", v);
}

fn main() {
    let args = Args::parse();

    let x_token = args.x_token;
    x_token.map(|token| println!("{}", token));
    // for _ in 0..args.count {
    //     println!("Hello {:?}!", &x_token);
    // }

    let mut a: SMap = HashMap::new();
    a.insert("k1".to_string(), "v2".to_string());

    foo(&a);
}
