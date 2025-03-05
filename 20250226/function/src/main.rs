fn main() {
    println!("Hello, world!");
    another_function();

    let mut number  = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;

    number = if condition { 3 } else { 4 };

    println!("The value of number is: {}", number);

    let mut count = 0;
    // 添加一个标签 'counting_up
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    let counter = loop_return();
    println!("-------------------\ncounter1 = {}", counter);
    println!("-------------------\ncounter2 = {}", loop_return2());

    for_list();
}

fn another_function() {
    println!("Another function");
}

fn loop_return() -> i32 {
    let mut counter = 0;
    // 表达是最后一定要有引号
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    result*3
}

fn loop_return2() -> i32 {
    let mut counter = 0;
    3*loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    }
}

fn for_list() {
    println!("---------for list----------");
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("The element in a is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("--------- end ----------");
}