/*
 * @Author: LeiJiulong
 * @Date: 2025-07-20 15:39:09
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-20 16:40:29
 * @Description:
 */

use actix_files as fs;
use actix_web::web::Data;
use actix_web::{App, Error, HttpResponse, HttpServer, Result, error, web};
use dotenv::dotenv;
use std::env;
use tera::Tera;


async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", "Bob");
    let s = tmpl.render("index.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    println!("hello iter1, ther server run in port: {}", addr);
    HttpServer::new(|| {
        // let s = concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter1/**/*");
        // println!("addr {}", s);
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter1/**/*")).unwrap();
        App::new()
            .app_data(Data::new(tera))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(&addr)?
    .run()
    .await
}
