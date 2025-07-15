/*
 * @Author: LeiJiulong
 * @Date: 2025-07-14 15:17:39
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-14 15:56:52
 * @Description: 
 */

use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");
    for stream in connection_listener.incoming() {
        let mut stream  = stream.unwrap();
        println!("Connection established");
        let mut buf = [0; 1024];
        let n = stream.read(&mut buf).unwrap();
        stream.write(&mut buf[0..n]).unwrap();
    }
}
