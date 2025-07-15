/*
 * @Author: LeiJiulong
 * @Date: 2025-07-08 16:37:52
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-10 18:51:51
 * @Description: 
 */
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use std::{collections::HashMap, sync::{Arc, Mutex}};
use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
#[warn(dead_code)]
type SharedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    // 监听指定地址，等待 TCP 连接进来
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Begin listening");
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // 第二个被忽略的项中包含有新连接的 `IP` 和端口信息
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    // `Connection` 对于 redis 的读写进行了抽象封装，因此我们读到的是一个一个数据帧frame(数据帧 = redis命令 + 数据)，而不是字节流
    // `Connection` 是在 mini-redis 中定义
    // let mut connection = Connection::new(socket);

    // if let Some(frame) = connection.read_frame().await.unwrap() {
    //     println!("GOT: {:?}", frame);

    //     // 回复一个错误
    //     let response = Frame::Error("unimplemented".to_string());
    //     connection.write_frame(&response).await.unwrap();
    // }

    use mini_redis::Command::{self, Get, Set};


    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                println!("insert  {}: {:?}",cmd.key().to_string() ,cmd.value().to_vec());
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    println!("get {}: {:?}", cmd.key(), value);
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        
        connection.write_frame(&response).await.unwrap();
    }


}

fn new_shared_db(num_shards: usize) -> SharedDb {
    let mut db  = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}


