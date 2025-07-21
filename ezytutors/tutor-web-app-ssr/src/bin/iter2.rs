/*
 * @Author: LeiJiulong
 * @Date: 2025-07-20 15:39:09
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-21 20:02:43
 * @Description:
 */

// use actix_files as fs;
use actix_web::web::Data;
use actix_web::{App, Error, HttpResponse, HttpServer, Result, error, web};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use tera::Tera;

#[derive(Deserialize, Serialize)]
pub struct Tutor {
    name: String,
}

async fn handle_post_tutor(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<Tutor>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", &params.name);
    ctx.insert("text", "Wecome");
    let s = tmpl
        .render("user.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s = tmpl
        .render("form.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    println!("hello iter2, ther server run in port: {}", addr);
    HttpServer::new(|| {
        // let s = concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter1/**/*");
        // println!("addr {}", s);
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter2/**/*")).unwrap();
        App::new()
            .app_data(Data::new(tera))
            .configure(app_config)
    })
    .bind(&addr)?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/tutors").route(web::post().to(handle_post_tutor))),
    );
}
