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

fn main() {
    let four = IpAddress {
        kind: IpAddrKind::V4,
        address: String::from("212.43.23.12"),
    };
    let six = IpAddress {
        kind: IpAddrKind::V6,
        address: String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"),
    };;
}
