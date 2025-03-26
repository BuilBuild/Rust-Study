/*
 * @Author: LeiJiulong
 * @Date: 2025-03-25 14:02:39
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-03-25 14:36:36
 * @Description: 
 */



fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background!", color);
    } else if is_tuesday {
        println!("Today is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color!");
        } else {
            println!("Using orange as the background color!");
        }
    } else {
        println!("Using orange as the background color!");
    }
    println!("============================================");

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    println!("============================================");

    let v = vec!['a', 'b', 'c'];
    
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    println!("============================================");

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched , y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
    println!("============================================");

    let x1 = 1;

    match x1 {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x2 = 5;
    match x2 {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x3 = 'c';

    match x3 {
        'a'..= 'j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    println!("============================================");

    
}
