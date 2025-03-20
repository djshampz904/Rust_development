#[derive(Debug)]
enum IpAddressKind {
    V4(u8, u8, u8, u8,),
    V6(String),
}

#[derive(Debug)]
struct IpAdd {
    ip_version: IpAddressKind,
}

enum Option<T> {
    None,
    Some(T),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
       // match self {
       //    Message::Write(text) => println!("{text}"),
       // }
    }
}

fn main() {
    let localhost = IpAdd {
        ip_version: IpAddressKind::V4(127, 0, 0, 1),
    };

    let loopback = IpAdd {
        ip_version: IpAddressKind::V6(String::from("::1"))
    };

    println!("Localhost: {:?}", localhost);
    println!("Loopback: {:?}", loopback);

    let m = Message::Write(String::from("Martin"));

    m.call();

    let some_value = 
}
