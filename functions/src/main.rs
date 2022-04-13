fn main() {
    let x = plus_one(1);

    println!("value of x: {}", x)
}

fn another_function(x: i32) {
    println!("This value of x is: {}", x);
}

fn print_labeled_mesuarement(x: i32, unit_label: char) {
    println!("The measuarement is: {}{}", x, unit_label);
}

fn expression(){
    let y = {
        let x = 3;
        x + 1
    };

    println!("This value of y is: {}", y);

    let x = five();

    println!("five function: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}