/*
 * @Author: LeiJiulong
 * @Date: 2025-07-13 10:47:42
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-13 11:15:12
 * @Description: 
 */
fn main() {
    if let Err(e) = wcr::get_args().and_then(wcr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
