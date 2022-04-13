Standard library input/output
```rust
use std::io;
```

To initiate the program have to use **main** function
```rust
fn main() {
```

Storing values to a varible, very diferent for other high level languagues 
```rust
let mut guess = String::new(); 
```

Remember that if I don't declarate **mut** the variable will be immutable.

To Receive User Input use stdin functionality from input/output library.
```rust
    io::stdin()
        .read_line($mut guess)
        .expect("Failed to read line");
    
    // I also can do that.

    io::stdin().read_line($mut guess).expect("Failed to read line");

    // Is the same thing but using just one line of code.
```

To use crate just go to the **Cargo.toml file** and add the crate bellow the **[dependecies]**

```rust
[dependecies]
rand = "0.8.3"
```

To Rust accept an Int variable we use this code
```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

To loop just use **loop** and a condition do get out like a match expression with a break, example:
```rust
loop {
    // Snip code
    match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too Small!"),
    Ordering::Greater => println!("Too Big"),
    Ordering::Equal => {
        println!("You win");
        break;
    }
}
```

Have many other things but I don't wanna a copy paste is just anotations...