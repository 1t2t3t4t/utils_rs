use utils_rs::derive::WithChain;

#[derive(Debug, Default, WithChain)]
struct Test {
    name: String,
    age: i32,
}

fn main() {
    let mut t = Test::default();
    t = t.with_name("Boss").with_age(39);
    println!("Hello, world! {:?}", t);
}
