/*
 * @Author: LeiJiulong
 * @Date: 2025-07-16 12:27:01
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-16 15:14:50
 * @Description: 
 */

use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;
#[path = "../models.rs"]
mod models;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()>{
    let shared_data = web::Data::new(AppState{
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });
    let app = move || {
        App::new().app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };
    
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await

}
