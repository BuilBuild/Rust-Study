/*
 * @Author: LeiJiulong
 * @Date: 2025-07-16 12:29:29
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-16 12:29:31
 * @Description: 
 */

use std::sync::Mutex;
use super::models::Course;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>,
}