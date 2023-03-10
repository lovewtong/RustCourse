// 元组

use std::io;

// 元组是由多种类型组合到一起形成的，因此它是复合类型，元组的长度是固定的，元组中元素的顺序也是固定的。
fn main() {
    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let (x,y,z) = tup;

    // println!("the value of x is {}",x);
    // println!("the value of y is {}",y);
    // println!("the value of z is {}",z);

    // let _five_hundred = tup.0;

    // let _six_point_four = tup.1;

    // let num = tup.2;

    // println!("num is {}",num);

    // let s1 = String::from("Hello");
    // let (s2,len) = calculate_length(s1);
    // println!(
    //     "the length of '{}' is {}.",s2,len
    // );

    let a = [1, 2, 3, 4, 5];

    println!("please enter the number");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    )
}

// fn calculate_length(s: String) -> (String,usize) {
//     let length = s.len();

//     (s,length)
// }

// 在其他语言中，可以用结构体来声明一个三维空间中的点，例如 Point(10, 20, 30)，虽然使用 Rust 元组也可以做到：(10, 20, 30)
// 但是这样写有个非常重大的缺陷：不具备任何清晰的含义
