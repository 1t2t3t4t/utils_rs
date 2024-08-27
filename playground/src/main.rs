use utils_rs::derive::WithChain;

#[derive(Debug, Default, WithChain)]
struct Test {
    name: String,
}

fn main() {
    println!("Hello, world!");
}
