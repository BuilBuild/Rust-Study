/*
 * @Author: LeiJiulong
 * @Date: 2025-07-09 16:56:12
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-10 14:04:12
 * @Description: 
 */
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {

    let mut f = File::open("foo.txt").await?;
    let mut buf = [0; 10];
    let n = f.read(&mut buf).await?;

    println!("The bytes is: {:?}", &buf[..n]);

    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;

    println!("{:?}", buffer);

    let mut f = File::create("foo2.txt").await?;
    let n = f.write(b"hello world").await?;
    println!("write num = {}", n);
    
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo3.txt").await?;
    io::copy(&mut reader, &mut file).await?;

    Ok(())
}
