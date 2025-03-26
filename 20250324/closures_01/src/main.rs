/*
 * @Author: LeiJiulong
 * @Date: 2025-03-24 01:57:08
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-03-24 01:59:43
 * @Description: 
 */
use std::thread;
use std::time::Duration;

fn main() {
    let s = simulated_expensive_calculation(2);
    println!("{}", s);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

