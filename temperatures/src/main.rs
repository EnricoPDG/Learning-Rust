use std::io;

fn main() {
    println!("Converting temperatures!");

    let mut temperature = String::new();

    println!("Enter '1' if you want convert Fahrenheit to Celsius or '2' if you want to convert Celsius to Fahrenheir.");

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read. ;(");
    
    let temperature: u8 = temperature.trim().parse().expect("Valus was not a number");

    println!("Now input the value: ");
    
    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read. ;(");

    let _value: f32 = value.trim().parse().expect("Value was not a number");

    let result: f32;
    if temperature == 1 {
        result = 1.8 * _value + 32.0;
        println!("{}", result)
    } else {
        result = (_value - 32.0) / 1.8;
        println!("{}", result)
    }
}
