fn main() {
    //0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987
    let mut cont:u8 = 0;
    let mut number:u128 = 1;
    let mut aux:u128 = 0;
    let mut result:u128;

    loop {
        if cont == 10 { break }

        println!("{}", number);
        result = number + aux;
        aux = number;
        number = result;

        cont +=1;
    }
}
