enum IpAddrKind {
    V6(u8, u8, u8, u8),
    V4(String)
}

impl IpAddrKind {
    fn what_am_i(&self) {
        match self {
            IpAddrKind::V6(a,b,c,d) => println!("I am a V6 addess! {}.{}.{}.{}", a,b,c,d),
            IpAddrKind::V4(a) => println!("I am a V4 addess! {}>", a)
        }
    }
}

enum Gear {
    Cam{color: String, size: u8}, // struct
    Chock(u8), // int
    Biner(Option<u8>), // Option enum
    Rope(u8, String), // Tuple
}

fn main() {
    let localhost = IpAddrKind::V6(127, 0, 0, 1);
    let _loopback = IpAddrKind::V4(String::from("::1"));

    localhost.what_am_i()


}
