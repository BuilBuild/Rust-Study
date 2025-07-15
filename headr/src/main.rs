/*
 * @Author: LeiJiulong
 * @Date: 2025-07-12 16:57:04
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-12 20:05:35
 * @Description: 
 */
fn main() {
    if let Err(e) = headr::get_args().and_then(headr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    println!("Hello, world!");
}
