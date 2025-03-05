/*
 * @Author: LeiJiulong
 * @Date: 2025-03-03 17:55:46
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-03-04 13:00:27
 * @Description: 
 */


fn main() {
    let mut s1 = String::from("hello"); // s 进入作用域

    let c = " world!";
    s1.push_str(c);
    println!("s = {}", s1);
    println!("c = {}", c);

    let s2 = s1; // s1 被移动到 s2
    // println!("s1 = {}", s1); // 错误！s1 已经被移动
    println!("s2 = {}", s2);

    let s3 = s2.clone(); // s2 被克隆到 s3
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);

    let mut s4 = String::from("hello");
    let s41 = &s4;
    let s42 = &s4;
    println!("s41 = {}", s41);
    println!("s42 = {}", s42);
    

    println!("s41= {} s42= {}", s41, s42);
    let s43 = &mut s4;
    println!("s43 = {}", s43);
}
