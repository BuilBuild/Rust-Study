/*
 * @Author: LeiJiulong
 * @Date: 2025-07-18 12:45:58
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-18 13:46:05
 * @Description: 
 */

use crate::errors::EzyTutorError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    Ok(HttpResponse::Ok().json(&response))
}

