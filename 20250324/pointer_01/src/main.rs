/*
 * @Author: LeiJiulong
 * @Date: 2025-03-24 12:30:39
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-03-24 19:19:32
 * @Description: 
 */
use std::ops::Deref;
use std::mem::drop;
use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let x = 5;
    let y = MyBox::new(5);
    assert_eq!(x, *y);
    let name = MyBox::new(String::from("Rust"));
    hello(&name);
    hello(&(*name)[..]);

    let c = CustomSmartPointer { data: String::from("my stuf") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
    drop(c);

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let e = Cons(3, Rc::clone(&a));
    println!("count after creating a = {}", Rc::strong_count(&a));

}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}




struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    // *(y.deref())
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}