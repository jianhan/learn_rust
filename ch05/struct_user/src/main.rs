struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user = User {
        username: String::from("johnsmith"),
        email: String::from("johnsmith@gmail.com"),
        sign_in_count: 12,
        active: false,
    };
    println!("Hello, world!");
}
