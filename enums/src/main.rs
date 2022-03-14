enum IpAddrKind {
    V6(u8, u8, u8, u8),
    V4(String)
}

fn main() {
    let localhost = IpAddrKind::V6(127, 0, 0, 1);
    let loopback = IpAddrKind::V4(String::from("::1"));


}
