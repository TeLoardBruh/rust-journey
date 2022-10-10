fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
    test_str();
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn test_str() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);
}
