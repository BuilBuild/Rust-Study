/*
 * @Author: LeiJiulong
 * @Date: 2025-07-21 00:42:33
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-21 20:01:10
 * @Description: 
 */
use crate::handler::*;
use actix_files as fs;
use actix_web::web;

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(show_register_form)))
            .service(web::resource("/register").route(web::post().to(handle_register))),
    );
}