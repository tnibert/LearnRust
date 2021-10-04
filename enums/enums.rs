enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr1 {
    kind: IpAddrKind,
    address: String,
}

// interesting...
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enums can also have methods
impl Message {
    fn call(&self) {
        // method body would be defined here
        // and apparently this will compile with an empty body
    }
}

fn main() {
    let home = IpAddr1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr1 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr2::V4(String::from("127.0.0.1"));

    let loopback = IpAddr2::V6(String::from("::1"));

    // Option enum lets null concept exist without having null
    let absent_number: Option<i32> = None;
    let present_number: Option<i32> = Some(5);
}