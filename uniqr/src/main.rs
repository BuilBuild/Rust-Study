/*
 * @Author: LeiJiulong
 * @Date: 2025-07-13 12:52:02
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-13 13:45:07
 * @Description:
 */
fn main() {
    if let Err(e) = uniqr::get_args().and_then(uniqr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}
