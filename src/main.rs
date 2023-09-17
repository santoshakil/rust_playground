#[tokio::main]
async fn main() {
    match 5.5 {
        n if is_even(n) => println!("z is Even"),
        n if is_odd(n) => println!("z is Odd"),
        n if is_number(n) => println!("z is a Number"),
        _ => println!("z is something else"),
    }
}

fn is_even(n: f32) -> bool {
    println!("is_even called");
    n % 2.0 == 0.0
}

fn is_odd(n: f32) -> bool {
    println!("is_odd called");
    n % 2.0 != 0.0
}

fn is_number(n: f32) -> bool {
    println!("is_number called");
    n.is_finite()
}
