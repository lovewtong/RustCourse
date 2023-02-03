// 函数

// rust是一门强语言，所以需要给每一个函数参数标识出它们具体的类型
// fn main() {
//     another_function(5, 6.1);
// }

// fn another_function(x:i32, y:f32) {

//     println!("the value of x is {}",x);
//     println!("the value of y is {}",y);
// }

fn plus_five(x:i32) -> i32 {
    x + 5
}

fn main() {
    let x = plus_five(5);

    println!("The value of x is: {}", x);
}
