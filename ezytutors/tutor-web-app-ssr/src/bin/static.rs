/*
 * @Author: LeiJiulong
 * @Date: 2025-07-20 15:15:26
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-21 20:02:15
 * @Description: 
 */

use actix_files as fs;
use actix_web::{ App, HttpServer}; 
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| {
        "127.0.0.1:8080".to_string()
    });

    
    println!("hello iter1, ther server run in port: {}", addr);
    HttpServer::new(|| {
        App::new().service(fs::Files::new(
            "/static", "./static",
        ).show_files_listing())
    })
    .bind(&addr)?
    .run()
    .await
}
