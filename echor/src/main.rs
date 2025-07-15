/*
 * @Author: LeiJiulong
 * @Date: 2025-07-11 18:23:08
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-12 12:41:10
 * @Description: 
 */
use clap::{App, Arg};
fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Leijiulong")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
            .short("n")
            .help("Do not print newling")
            .takes_value(false),
        )
        .get_matches();
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else {"\n"});
    // println!("{:?}", matches);
}
