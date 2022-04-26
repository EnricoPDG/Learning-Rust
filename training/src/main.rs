use std::option::Option;

fn main() {
    let five = mais_um(Some(5));   
    dbg!(five);
}

fn mais_um(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
