/*
 * @Author: LeiJiulong
 * @Date: 2025-03-24 23:06:48
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-03-25 00:19:19
 * @Description: 
 */

use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};
use std::rc::Rc;



fn main() {
    let handle1 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} in main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle1.join().unwrap();

    let v = vec![1, 2, 3 ,6];
    let handle2 = thread::spawn(move || {
        println!("Here is a vector {:?}", v)
    });
    handle2.join().unwrap();
    println!();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result = {}", *counter.lock().unwrap());
}
