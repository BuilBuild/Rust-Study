
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    #[allow(dead_code)] // 忽略 width 的未使用警告
    width: u32,
    #[allow(dead_code)] // 忽略 height 的未使用警告
    height: u32,
}
// impl 块中的所有内容都将与 Rectangle 类型相关联
// 所有在 impl 块中定义的函数被称为 关联函数（associated functions），因为它们与 impl 后面命名的类型相关
impl Rectangle {
    // & 来表示这个方法借用了 Self 
    // 如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self
    // object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。
    /*
    p1.distance(&p2);
    (&p1).distance(&p2);
     */
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = build_user(String::from("someone@example.com"), String::from("username123"));

    println_user(&user1);
    println_user(&user2);

    let user3 = User {
        // 这里用雷移动 之后不能再使用user2
        email: user2.email,
        username: user2.username,
        active: user2.active,
        sign_in_count: user2.sign_in_count
    };

    println_user(&user3);

    let user4 = User {
        email: String::from("someone@example.com"),
        ..user3
    };

    println_user(&user4);

    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);
    // println!("black is {:?}", black);
    // println!("origin is {:?}", origin);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    // 宏dbg! 宏接收一个表达式的所有权（与 println! 宏相反，后者接收的是引用），打印出代码中调用 dbg! 宏时所在的文件和行号，以及该表达式的结果值，并返回该值的所有权。
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn println_user(user: &User) {
    println!("user email is {}", user.email);
    println!("user username is {}", user.username);
    println!("user active is {}", user.active);
    println!("user sign_in_count is {}", user.sign_in_count);
}


