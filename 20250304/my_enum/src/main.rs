/*
 * @Author: LeiJiulong
 * @Date: 2025-03-04 15:50:41
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-03-04 15:58:05
 * @Description: 
 */
#[derive(Debug)]
 enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    println!("-------------------------------");
    
    let home = IpAddr{ 
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    }
}

fn route(ip_kind: IpAddrKind) {
    println!("route: {:?}", ip_kind);
}
