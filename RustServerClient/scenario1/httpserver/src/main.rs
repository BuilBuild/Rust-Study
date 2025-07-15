/*
 * @Author: LeiJiulong
 * @Date: 2025-07-14 15:17:39
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-16 00:00:24
 * @Description: 
 */

use httpserver::server::Server;

fn main() {
   let server = Server::new("localhost:3000");
   server.run();
}
