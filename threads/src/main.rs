use threads::validator::Validator;

fn main() {
    let validator = Validator::new();
    // validator.exit();
    validator.join();
}
