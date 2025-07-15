/*
 * @Author: LeiJiulong
 * @Date: 2025-07-10 15:50:03
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-10 15:54:23
 * @Description: 
 */

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()>{

    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut rd, mut wr) = io::split(socket);

    tokio::spawn(async move {
        wr.write_all(b"hello ").await?;
        wr.write_all(b"world").await?;
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];
    loop {
        let n = rd.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        println!("GOT {:?}", &buf[..n]);
    }


    Ok(())
}