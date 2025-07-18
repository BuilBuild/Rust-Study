/*
 * @Author: LeiJiulong
 * @Date: 2025-07-18 12:45:50
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-18 19:16:03
 * @Description: 
 */

use crate::dbaccess::course::*;
use crate::errors::EzyTutorError;
use crate::models::course::{Course, UpdateCourse};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub type MyResult = Result<HttpResponse, EzyTutorError>;


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

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_courese: web::Json<UpdateCourse>,
    path: web::Path<(i32, i32)>,
) -> MyResult {
    let (tutor_id, course_id) = path.into_inner();
    update_courese_details_db(
        &app_state.db,
        tutor_id,
        course_id,
         update_courese.try_into().unwrap())
    .await
    .map(|course|{HttpResponse::Ok().json(course)})
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> MyResult {
    let (tutor_id, course_id) = path.into_inner();
    
    delete_course_db(&app_state.db, tutor_id, course_id)
    .await
    .map(|resp| HttpResponse::Ok().json(resp))
    
}