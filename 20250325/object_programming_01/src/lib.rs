/*
 * @Author: LeiJiulong
 * @Date: 2025-03-25 00:27:41
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-03-25 14:01:27
 * @Description: 
 */

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new(list: Vec<i32>) -> AveragedCollection {
        let mut obj = AveragedCollection {
            list: list,
            average: 0.0,
        };
        obj.update_average();
        obj
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn get_average(&self) -> f64 {
        self.average
    }
    
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    } 

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


pub trait Draw {
    fn draw(&self);
}

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T: Draw> Screen<T> {
//     where T: Draw {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button draw!")
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn obj_test() {
        let mut s = AveragedCollection {
            list: vec![1, 2, 3],
            average: 0.0,
        };
        s.add(4);
        assert_eq!(2.5, s.get_average());
        let mut s2 = AveragedCollection::new(vec![1, 2, 3]);
        s2.add(4);
        assert_eq!(2.5, s.get_average());
    }
}
