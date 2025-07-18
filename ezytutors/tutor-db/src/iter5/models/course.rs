/*
 * @Author: LeiJiulong
 * @Date: 2025-07-18 12:44:16
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-18 13:33:37
 * @Description: 
 */

use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
    pub posted_time: Option<NaiveDateTime>,
}

// #[derive(Deserialize, Debug, Clone)]
// pub struct CreateCourse {
//     pub tutor_id: i32,
//     pub course_name: String,
//     pub course_description: Option<String>,
//     pub course_format: Option<String>,
//     pub course_structure: Option<String>,
//     pub course_duration: Option<String>,
//     pub course_price: Option<i32>,
//     pub course_language: Option<String>,
//     pub course_level: Option<String>,
// }

// #[derive(Deserialize, Debug, Clone)]
// pub struct UpdateCourse {
//     pub course_name: Option<String>,
//     pub course_description: Option<String>,
//     pub course_format: Option<String>,
//     pub course_structure: Option<String>,
//     pub course_duration: Option<String>,
//     pub course_price: Option<i32>,
//     pub course_language: Option<String>,
//     pub course_level: Option<String>,
// }