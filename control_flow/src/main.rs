fn main() {
   for_();
}

fn loop_(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

fn if_() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("Tha value of number is: {}", number)
}

fn other_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("{}", counter);

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn for_() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("finish");
}