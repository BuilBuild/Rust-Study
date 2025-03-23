#[derive(Debug)]
struct MyStruct {
    x: i32,
    y: i32,
}

fn main() {
    let i = 5;
    // println!("The value of i is: {}", i);
    test(i);
    println!("The value of i is: {}", i);

    let mut s = MyStruct {x: 1, y: 2};
    test2(&mut s);
    println!("The value of s is: ({}, {})", s.x, s.y);
    println!("------------------------------------");
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    println!("The first element is: {first}");

    v.push(6);
    println!("------------------------------------");
    let mut v = vec![100, 32, 57];
    print_vec(&v);
    for i in &mut v {
        *i += 50;
    }
    println!("after modify vec!");
    print_vec(&v);
    println!("------------------------------------");
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        match i {
            SpreadsheetCell::Int(i) => println!("Int: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            _ => (),
        }
    }
    println!("------------------------------------");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(" ");
    s1.push_str(s2);
    println!("s2 is {s2}");
    println!("s1 is {s1}");
    println!("------------------------------------");
    // (1) 直接操作引用指向的值
    // 当需要修改可变引用指向的值时，必须显式解引用： *y += 1;
    let mut x = 5;
    let y = &mut x;
    *y += 1; // 显式解引用修改值
    // (2) 匹配值的类型
    // 当上下文需要具体类型而非引用时，需显式解引用：
    let a = &42;
    let b = *a; // b 的类型是 i32，而非 &i32


}

fn test(i: i32) {
    println!("The value of i is: {}", i);
    
}

fn test2(s: &mut MyStruct) {
    println!("The value of s is: ({}, {})", s.x, s.y);
    s.x = 3;
}

fn print_vec(v: &Vec<i32>) {
    for i in v {
        println!("The value of i is: {}", i);
    }
}
