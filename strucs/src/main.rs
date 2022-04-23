struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email, //Can use or not the parameter
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("ricopapsch@Hotmail.com"), 
        String::from("EnricoPDG")
    );

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@email.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("onemore@email.com"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

