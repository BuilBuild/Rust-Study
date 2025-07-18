/*
 * @Author: LeiJiulong
 * @Date: 2025-07-17 12:01:48
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-17 14:45:27
 * @Description:
 */

use actix_web::{HttpResponse, web};

use super::db_access::*;
use super::errors::EzyTutorError;
use super::models::Course;
use super::state::AppState;

pub type MyResult = Result<HttpResponse, EzyTutorError>;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> MyResult {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    Ok(HttpResponse::Ok().json(&response))
}

pub async fn post_new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> MyResult {
    post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> MyResult {
    let tutor_id = path.into_inner();
    get_courses_for_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> MyResult {
    let (tutor_id, course_id) = path.into_inner();
    get_course_details_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}
