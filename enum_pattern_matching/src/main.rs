fn main() {
    enum1();
    enum2();
    enum3();   
    match_();
}

fn enum1() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.01"));

    let loopback = IpAddr::V4(String::from("::1"));
}

fn enum2() {    
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        adress: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        adress: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        adress: String::from("::1"),  
    };
}

fn enum3() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
}

fn option(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => dbg!(Some(i + 1)),
    }
}

fn match_() {
    let config_max = Some(3u8);
    
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }      
}