Decoding this code:
`use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}`

The library io comes from the standard library, knows as `std`, to bring the to the code use `std::io;`, the io library contains many thinks I will need when doing input and output. The core part of the module is te **Read** and **Write** traits.

Rust come with a variety of things in its standard library.

The *main* functions is the entry point into the program:

`fn main() {}`

`println!` is a macro that print a string to the screen:
`
println!("Guess the number!");
println!("Please input your guess");
`

**Storing Values with Variable**

Crating a *variable* to store the user input:
`
     let mut guess = String::new();
`

*let* statement to create the variable

In Rust, variable are immutable by default. To make a variable mutable, just add *mut* before the variable name:
`
let test = 3; // imutable
ket mut test2 = 4; // mutable
`
`String::new`, a function that return a new instance of a *String*. *String* is a string type provided by the standard library that is growable, UTF-8 encoded bit of text.

The *::* syntax in the `::new` line indicates tha *new* is an associated function of the *String* type. An associated function is a function that's implemented on a type, in this case String. This *new* function creates a new, empty string.

In full, the `let mut guess = String::new();` line has created a mutable variable that is currently bound to a new, empty instance of a String

**Receiving User Input**

Recall that I included the input/output functionality from the standart library, so now call the *stdin* function from the *io* module.
`
    io::stdin()
        .read_line(&mut guess)
`
The line *.read_line(&mut guess)* calls the *read_line* method on the standard input handle to get input from the user. I'am also passing *&mut guess* as the argument to *read_line* to tell it what string to store the user input in. The full job of *read_line* is to take whatever the user types into standard input and append that into a string (without overwriting its contents).

The *&* indicates that this argument is a reference, which gives me a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.

**Handling With Potential Failure with the Result Type**

Result’s variants are *Ok* or *Err*. The *Ok* variant indicates the operation was successful, and inside *Ok* is the successfully generated value. The *Err* variant means the operation failed, and *Err* contains information about how or why the operation failed.

So *.expect* is expecting a error and if expect is ok they don't run the code *"Failed to read line"*.

**Printing Values with println! Placeholders**

This line prints the string that now contains the user’s input. The {} set of curly brackets is a placeholder: think of {} as little crab pincers that hold a value in place. You can print more than one value using curly brackets: the first set of curly brackets holds the first value listed after the format string, the second set holds the second value, and so on. Printing multiple values in one call to println! would look like this:
`
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
`