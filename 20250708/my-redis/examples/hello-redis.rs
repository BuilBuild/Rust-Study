/*
 * @Author: LeiJiulong
 * @Date: 2025-07-08 13:20:38
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-08 16:33:29
 * @Description: 
 */

use mini_redis::{client, Result};

async fn say_to_world() -> String {
    String::from("world")
}

#[tokio::main]
async fn main() -> Result<()> {
    // 建立与mini-redis服务器的连接
    let mut client  = client::connect("127.0.0.1:6379").await?;

    // 设置key 
    client.set("hello", "world".into()).await?;

    // 获取key值
    let result = client.get("hello").await?;
    println!("从服务器获取到结果={:?}", result);
    let op = say_to_world();
    println!("{}", op.await);

    Ok(())
}
