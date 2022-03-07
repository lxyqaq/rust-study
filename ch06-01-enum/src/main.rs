fn main() {

    let ipv4 = IpAddress::Ipv4(String::from("127.0.01"));

    let ipv6 = IpAddress::Ipv6(String::from("::1"));

    println!("ipv4 = {:?}", ipv4);

    println!("ipv6 = {:?}", ipv6);

}

#[derive(Debug)]
enum IpAddress {
    Ipv4(String),
    Ipv6(String),
}
