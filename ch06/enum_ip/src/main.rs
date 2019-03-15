#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrKindV2 {
    V4(String),
    V6(String),
}

enum IpAddrV3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let four = IpAddress {
        kind: IpAddrKind::V4,
        address: String::from("212.43.23.12"),
    };
    let six = IpAddress {
        kind: IpAddrKind::V6,
        address: String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"),
    };;

    let home = IpAddrKindV2::V4(String::from("127.0.0.1"));

    let loopback = IpAddrKindV2::V6(String::from("::1"));

    let home = IpAddrV3::V4(127, 0, 0, 1);

    let loopback = IpAddrV3::V6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!("{} {}", some_number, some_string);
}
