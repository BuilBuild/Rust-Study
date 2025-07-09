/*
 * @Author: LeiJiulong
 * @Date: 2025-03-27 16:23:13
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-03-27 16:46:30
 * @Description: 
 */


fn main() {
    let a: u16 = 50115;
    let b: i16 = -15421;
    
    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);

    let a: f32 = 42.42;
    // 42.42 f32所产生的位模式视为一个10进制数 std::mem::transmute(a) 转换为左边声明的类型
    let frankentype: u32 = unsafe { std::mem::transmute(a) };
    println!("frankentype {}  {:032b}", frankentype, frankentype);
    
    let b: f32 = unsafe { std::mem::transmute(frankentype) };
    println!("b {}", b);
    assert_eq!(a, b);

    // 字节序
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];
    println!("big_endian {:?}    little_endian {:?}", big_endian, little_endian);

    
}
