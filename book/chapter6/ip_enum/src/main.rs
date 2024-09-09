enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String), // IpAddrKind::V6() becomes a function that takes a String argument and returns an instance of IpAddr
}

fn main() {
    //let four = IpAddrKind::V4;
    //let six = IpAddrKind::V6;

    //route(IpAddrKind::V4);
    //route(IpAddrKind::V6);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

}

//fn route(ip_kind: IpAddrKind) {}