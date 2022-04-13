use std::io;

fn main() {
    test_array();
    data_type();
}

fn data_type() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("{}, {}, {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{}, {}, {}", five_hundred, six_point_four, one);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("this is the first value {}\nThis is the second value {}", a[0], a[1]);

    let b = [3; 5];

    println!("{}, {}, {}, {}, {}", b[0], b[1], b [2], b[3], b[4]);
}

fn test_array() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at the index {} is: {}",
        index, element
    );    
}
