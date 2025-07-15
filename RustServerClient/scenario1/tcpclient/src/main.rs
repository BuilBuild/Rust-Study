
use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    stream.write("hello world".as_bytes()).unwrap();
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).unwrap();
    println!("{:>4?}", str::from_utf8(&buf[0..n]).unwrap());
}
