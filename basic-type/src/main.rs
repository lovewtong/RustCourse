// 基本类型

// return的应用，在代码块里面，如果没有return字段，只会视为运行，而不是一个结果
// fn plus_or_minus(x:i32) -> i32 {
//     if x > 5 {
//         return x - 5
//     }

//     return x + 5
// }

// fn main() {
//     let x = plus_or_minus(5);

//     let _y = 256_u8;
//     println!("The value of x is: {}", x);
// }


fn main() {
    let a : u8 = 255;

    let b = a.wrapping_add(20);
    println!("{}", b);  // 19

    let _c = a.checked_add(20);
    println!("{}", b);
}
