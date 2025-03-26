/*
 * @Author: LeiJiulong
 * @Date: 2025-03-25 01:25:10
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-03-25 16:48:48
 * @Description: 
 */

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;
use std::thread;

use web_app::ThreadPool;


fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
            println!("Connection established!");
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    use std::time::Duration;
    thread::sleep(Duration::from_secs(5));
    let mut buffer = [0; 1024];
    
    stream.read(&mut buffer).unwrap();
    
    
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}