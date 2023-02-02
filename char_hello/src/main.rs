// fn main() {
//     println!("Hello, world!");
// }

// rust字符占用永远为4，字符串占用永远为16.rust字符相对开放一点，所有的 Unicode 值都可以作为 Rust 字符。
// fn main() {
//     let c = '😊';
//     let d = 'ℤ';
//     let e = '中';
//     let c1 = "😊😊😊";
//     let e1 ="中中中中中";
//     let e2 ="中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中";
    
//     println!("{}",c);

//     println!("字符{}占用了{}字节的内存大小",d,std::mem::size_of_val(&d));
//     println!("字符{}占用了{}字节的内存大小",e,std::mem::size_of_val(&d));
//     println!("字符{}占用了{}字节的内存大小",c1,std::mem::size_of_val(&c1));
//     println!("字符{}占用了{}字节的内存大小",e1,std::mem::size_of_val(&e1));
//     println!("字符{}占用了{}字节的内存大小",e2,std::mem::size_of_val(&e2));
// }

// 布尔值，一般用于结束程序，判断
// fn main() {
//     let _t = true;
//     let f:bool = false;
//     if f{
//         println!("假");
//     }
// }


//练习
// 1.修改2处 `assert_eq!` 让代码工作
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),1); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),3); 

    println!("Success!")
}

// fn main() {
    
// }