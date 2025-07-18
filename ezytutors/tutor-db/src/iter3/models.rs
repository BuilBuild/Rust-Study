/*
 * @Author: LeiJiulong
 * @Date: 2025-07-16 23:25:11
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-16 23:39:53
 * @Description: 
 */
use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}
impl From<web::Json<Course>> for Course {
    fn from(tweet: web::Json<Course>) -> Self {
        Course {
            course_id: tweet.course_id,
            tutor_id: tweet.tutor_id,
            course_name: tweet.course_name.clone(),
            posted_time: tweet.posted_time,
        }
    }
}
