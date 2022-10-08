use std::io;

fn main() {
    println!("Welcome to Convert temperatures between Fahrenheit and Celsius");

    let mut convert_temp = String::new();
    let msg = "Fail cannot input this";
    io::stdin().read_line(&mut convert_temp).expect(msg);

    let convert_temp: f32 = match convert_temp.trim().parse() {
        Ok(v) => v,
        Err(_) => 0.0,
    };

    let f_temp = (convert_temp * 1.8) + 32.0;

    println!("From C to F is {f_temp}");
}
