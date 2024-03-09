#[derive(Debug)]
enum IpAddressKind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddressKindV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

fn main() {
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;


    let home = IpAddressKind::V4(String::from("127.0.0.1"));

    // let loopback = IpAddress {
    //     kind: IpAddressKind::V6,
    //     address: String::from("::1"),
    // };

    let loopback = IpAddressKind::V6(String::from("::1"));

    println!("home is {home:?}");
    println!("loopback is {loopback:?}");

    let home_v2 = IpAddressKindV2::V4(127, 0, 0, 1);
    let loopback_v2 = IpAddressKindV2::V6(String::from("::1"));

    println!("home v2 is {home_v2:?}");
    println!("loopback is {loopback_v2:?}");

    let v = IpAddressKind::V6(String::from("value"));

    match v {
        IpAddressKind::V6(value) if value == "hello" => println!("Hello {}", value),
        IpAddressKind::V4(value) => println!("V4 {}", value),
        _ => println!("None"),
    }
}

fn route(ip_kind: IpAddressKind) {
    println!("ip kind is {ip_kind:?}");
}