enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr{
    kind : IpAddrKind,
    address : String
}

fn main() {
    // println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // fn route(ip_kind : IpAddrKind){}

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    let home = IpAddr{
        kind : IpAddrKind::V4,
        address : String::from("127.00.0.1")
    }

    let loopback = IpAddr{
        kind : IpAddrKind::V6,
        address: String::from(":::1")
    }
}