/*
 * @Author: LeiJiulong
 * @Date: 2025-07-16 23:25:04
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-17 00:04:38
 * @Description: 
 */

use super::db_access::*;
use super::models::Course;
use super::state::AppState;

use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async  fn get_course_for_tutor(
    _app_state: web::Data<AppState>,
    _params: web::Path<i32>
) -> HttpResponse {
    let tuple = _params.into_inner();
    let tutor_id = tuple;
    let courses = get_courses_for_tutor_db(&_app_state.db, tutor_id).await;
    HttpResponse::Ok().json(courses)
}

pub async fn get_course_details(
    _app_state: web::Data<AppState>,
    _params: web::Path<(i32, i32)>
) -> HttpResponse {
    let (tutor_id, course_id) = _params.into_inner();
    let course = get_course_details_db(&_app_state.db, tutor_id, course_id).await;

    HttpResponse::Ok().json(course)
}

pub async fn post_new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>
) -> HttpResponse {
    let course = post_new_course_db(&app_state.db, new_course.into()).await;
    HttpResponse::Ok().json(course)
}