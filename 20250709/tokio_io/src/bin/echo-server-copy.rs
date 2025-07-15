/*
 * @Author: LeiJiulong
 * @Date: 2025-07-10 14:06:15
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-10 16:07:02
 * @Description: 
 */

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;


    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        println!("response {:?}", &buf[..n]);
                        if socket.write_all(&buf[..n]).await.is_err() {
                            return ;
                        }
                    }
                    Err(_) => {
                        return ;
                    }
                }
                
            }
        });
    }

}