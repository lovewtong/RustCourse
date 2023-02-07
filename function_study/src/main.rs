// 函数

// rust是一门强语言，所以需要给每一个函数参数标识出它们具体的类型
// fn main() {
//     another_function(5, 6.1);
// }

// fn another_function(x:i32, y:f32) {

//     println!("the value of x is {}",x);
//     println!("the value of y is {}",y);
// }


// 1.在rust中，函数也是表达式
// 2.x + 5 没有分号，因为它是表达式
// fn plus_five(x:i32) -> i32 {
//     x + 5
// }

// fn main() {
//     let x = plus_five(5);

//     println!("The value of x is: {}", x);
// }


// 简单的逻辑判断
// fn plus_or_minus(x:i32) -> i32 {
//     if x > 5 {
//         return x - 5
//     }

//     x + 5
// }

// fn main() {
//     let x = plus_or_minus(5);

//     println!("The value of x is: {}", x);
// }

// Rust中的特殊返回类型

// 隐式的表达了返回()
// #![allow(unused)]
// fn main() {
// use std::fmt::Debug;

// fn report<T: Debug>(item: T) {
//   println!("{:?}", item);

// }
// }


// 显式的表达了返回()
// #![allow(unused)]
// fn main() {
// fn clear(text: &mut String) -> () {
//   *text = String::from("");
// }
// }

// 报错原因:值的类型不一样，表达式与u32不一致
// 只有表达式才能返回值，而;结尾的是语句
// #![allow(unused)]
// fn main() {
// fn add(x:u32,y:u32) -> u32 {
//     x + y;
// }
// }

// 永不返回的发散函数！当！当作函数返回类型时，表示该函数永不返回，这种语法往往用做会导致程序崩溃的函数:
// #![allow(unused)]
// fn main(){
//     fn dead_end() -> ! {
//         panic!("你已经到了穷途末路，崩溃吗？")
//     }
// }

// 无限循环函数也会导致函数永不返回
// #![allow(unused)]
// fn main(){
//     fn forever() -> !{
//         loop {
//             //...
//         };
//     }
// }

// practice
// 1.*** 分清表达式与函数的区别，以及值的类型需要一致
// fn main() {
//     // 不要修改下面两行代码!
//     let (x, y) = (1, 2);
//     let s = sum(x, y);

//     assert_eq!(s, 3);
// }

// fn sum(x, y: i32) {
//     x + y;
// }

// fn main() {
//     // 不要修改下面两行代码!
//     let (x, y) = (1, 2);
//     let s = sum(x, y);

//     assert_eq!(s, 3);
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

// 2.** 类型
// fn main() {
//     print();
//  }
 
//  // 使用另一个类型来替代 i32
//  fn print() -> i32 {
//     println!("hello,world");
//  }
 
fn main() {
    print();
 }
 
 // 使用另一个类型来替代 i32
 fn print() -> () {
    println!("hello,world");
 }
 