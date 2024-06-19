use std::io;

fn main() {
    println!("Temperature conversion calculator");
    println!("Please select an option:");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    // get which conversion
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read input");
    let option: i32 = option.trim().parse().expect("Index entered was not a number");
    // get temperature
    println!("Please enter the temperature you want to convert: ");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");
    let temp: f64 = temp.trim().parse().expect("Temperature entered was not a number");
    if option == 1 {
        println!("{:.2} celsius is {:.2} fahrenheit.", temp, c_to_f(temp));
    } else if option == 2 {
        println!("{:.2} fahrenheit is {:.2} celsius.", temp, f_to_c(temp));
    } else {
        println!("Please enter a valid conversion option!");
    }
}

fn c_to_f(value: f64) -> f64 {
    (value * 1.8) + 32.0
}

fn f_to_c(value: f64) -> f64 {
    (value - 32.0) / 1.8
}