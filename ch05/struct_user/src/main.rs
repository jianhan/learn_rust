#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}

fn main() {
    let user = User {
        username: String::from("johnsmith"),
        email: String::from("johnsmith@gmail.com"),
        sign_in_count: 12,
        active: false,
    };
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user3 = build_user(
        String::from("someone@example.com"),
        String::from("test1@gmail.com"),
    );

    let r = Rectangle {
        width: 100,
        height: 100,
    };
    println!("{} {:?} {:#?}", area(&r), r, r);
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        active: false,
    }
}

fn area(r: &Rectangle) -> u64 {
    r.height * r.width
}
