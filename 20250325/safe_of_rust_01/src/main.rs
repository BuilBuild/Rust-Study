/*
 * @Author: LeiJiulong
 * @Date: 2025-03-25 14:53:53
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-03-25 15:10:50
 * @Description: 
 */

/*  unsafe
1. 解引用裸指针
2. 调用不安全的函数或方法
3. 访问或修改可变静态变量
4. 实现不安全的trait
5. 访问union的字段
*/
fn main() {
    println!("============================================");
    /*
    裸指针和智能指针的区别：
    1. 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
    2. 不保证指向有效的内存
    3. 允许为空
    4. 不能实现任何自动清理的功能
     */
    // 通过引用创建裸指针
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num  as *mut i32;
    // 创建指向任意内存地址的裸指针
    let address = 0x012345usize;
    let r = address as *const i32;

    // unsafe 块中解引用裸指针
    // 创建一个裸指针不危险，只有当访问其指向的值才有可能遇到无效值
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    
    // 调用不安全函数的方法 
    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {}
