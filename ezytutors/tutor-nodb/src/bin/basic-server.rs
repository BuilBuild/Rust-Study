/*
 * @Author: LeiJiulong
 * @Date: 2025-07-16 11:31:51
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-16 11:41:12
 * @Description: 
 */

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// Configure route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// Configure handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let app =  move || App::new().configure(general_routes);
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await

}
