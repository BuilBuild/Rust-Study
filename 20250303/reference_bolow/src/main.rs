/*
 * @Author: LeiJiulong
 * @Date: 2025-03-04 10:28:44
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-03-04 12:39:15
 * @Description: 
 */



fn main() {
    let s1 = String::from("hello");
    
    let s1_len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, s1_len);

    let mut s2 = String::from("hello");

    push_world(&mut s2);

    println!("{}", s2);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn push_world(s: &mut String) {
    s.push_str(" world!");
}