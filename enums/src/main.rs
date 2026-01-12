enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn print_ip_address(ip: IpAddr) {
    match ip {
        IpAddr::V4(p1, p2, p3, p4) => println!("IPv4 Address: {}.{}.{}.{}", p1, p2, p3, p4),
        IpAddr::V6(address) => println!("IPv6 Address: {}", address),
    }
}

fn main() {
    let number1 = 3;
    let number2: Option<i32> = Some(5);

    println!("add two i32: {}", number2.unwrap() + number1);
    

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    print_ip_address(home);
    print_ip_address(loopback);

    println!("Hello, world!");
}
