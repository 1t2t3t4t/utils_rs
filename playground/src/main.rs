use utils_rs::derive::WithChain;

#[derive(Debug, Default, WithChain)]
struct Test {
    name: String,
    age: i32,
    bx: Box<String>,
    arr: Vec<f64>,
}

fn main() {
    let mut t = Test::default();
    let x = t
        .with_name("Boss")
        .with_age(39)
        .with_bx("val".to_string())
        .with_arr(vec![1f64, 2f64]);
    println!("Hello, world! {:?}", x);
}
