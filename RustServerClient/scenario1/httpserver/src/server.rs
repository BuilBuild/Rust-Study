/*
 * @Author: LeiJiulong
 * @Date: 2025-07-15 16:16:02
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-15 23:58:15
 * @Description: 
 */

use super::router::Router;
use http::httprequests::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

//-------------------------------------------------
pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);

        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buffer = [0; 90];
            let n = stream.read(&mut read_buffer).unwrap();
            let req: HttpRequest = String::from_utf8(read_buffer[0..n].to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}