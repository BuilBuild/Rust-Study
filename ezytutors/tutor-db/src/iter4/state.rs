/*
 * @Author: LeiJiulong
 * @Date: 2025-07-17 12:01:59
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-17 12:14:15
 * @Description: 
 */

use sqlx::postgres::PgPool;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub db: PgPool,
}