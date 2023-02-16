trait A {
    fn foo_a() {
        println!("a");
    }
}
trait B: A {
    fn foo_b() {
        println!("b");
    }
}

struct S;

impl A for S {
    fn foo_a() {
        println!("a for s");
    }
}

// Commenting this line will result in a "trait bound unsatisfied" error
impl B for S {
    fn foo_b() {
        println!("b for s");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        S::foo_a();
        S::foo_b();
    }
}
