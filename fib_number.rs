fn main() {
    fib(10);
    let test: u32 = fibonacci(1);
    println!("{test}");
}

fn fib(number: u32) {
    for item in 0..number {
        println!("{item}");
    }
    println!("LIFTOFF!!!");
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
