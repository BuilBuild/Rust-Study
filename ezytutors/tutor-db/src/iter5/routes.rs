/*
 * @Author: LeiJiulong
 * @Date: 2025-07-18 12:31:03
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-18 13:16:11
 * @Description: 
 */

use crate::handlers::{course::*, general::*};
// use actix_web::web;

// pub fn general_routes(cfg: &mut web::ServiceConfig) {
//     cfg.route("/health", web::get().to(health_check_handler));
// }

// pub fn course_routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/courses")
//         .route("/", web::post().to(post_new_course))
//         .route("/{tutor_id}", web::get().to(get_courses_for_tutor))
//         .route("/{tutor_id}/{course_id}", web::get().to(get_course_details))
//         ,
//     );
// }