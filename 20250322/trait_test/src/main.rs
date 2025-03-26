/*
 * @Author: LeiJiulong
 * @Date: 2025-03-22 23:16:34
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-03-22 23:54:17
 * @Description:
 */
#[derive(Debug)]
struct Point <T, U> {
    #[allow(dead_code)]
    x: T,
    #[allow(dead_code)]
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 45, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![34, 45, 25, 1000, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let integer_and_float = Point {x: 5.2, y: 4.0};
    println!("The point is {:?}", integer_and_float);
    println!("The point x is {}", integer_and_float.x());
    println!("distance is {}", integer_and_float.distance_from_origin());

    let int_and_int = Point {x: 3, y: 8};

    let res_point = integer_and_float.mixup(int_and_int);
    println!("The  mixup point is {:?}", res_point);

}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn largest_T<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
