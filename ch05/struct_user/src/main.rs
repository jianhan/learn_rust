#[derive(Debug)]
struct Address {
    street_number: u64,
    street_name: String,
    suburb: String,
    state: String,
    country: String,
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    address: Address,
}

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}

fn main() {
    let a = Address {
        street_number: 12,
        street_name: String::from("smith street"),
        suburb: String::from("King"),
        state: String::from("QLD"),
        country: String::from("Australia"),
    };

    let user = User {
        username: String::from("johnsmith"),
        email: String::from("johnsmith@gmail.com"),
        sign_in_count: 12,
        active: false,
        address: a,
    };

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
        address: a,
    };

    user1.email = String::from("anotheremail@example.com");

    let user3 = build_user(
        String::from("someone@example.com"),
        String::from("test1@gmail.com"),
        a,
    );

    let r = Rectangle {
        width: 100,
        height: 100,
    };
    println!("{} {:?} {:#?}", area(&r), r, r);
}

fn build_user(username: String, email: String, address: Address) -> User {
    User {
        username,
        email,
        address,
        sign_in_count: 0,
        active: false,
    }
}

fn area(r: &Rectangle) -> u64 {
    r.height * r.width
}
