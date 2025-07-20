/*
 * @Author: LeiJiulong
 * @Date: 2025-07-21 00:42:54
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-21 00:53:52
 * @Description: 
 */

use sqlx::postgres::PgPool;

pub struct AppState {
    pub db: PgPool,
}

