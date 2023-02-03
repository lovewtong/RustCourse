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
// use std::mem::size_of_val;
// fn main() {
//     let c1 = 'a';
//     assert_eq!(size_of_val(&c1),4); 

//     let c2 = '中';
//     assert_eq!(size_of_val(&c2),4); 

//     println!("Success!")
// }

// 2.修改一行让代码正常打印
// fn main() {
//     let c1 = '中';
//     print_char(c1);
// } 

// fn print_char(c : char) {
//     println!("{}", c);
// }

// 3.使成功打印
// fn main() {
//     let _f: bool = false;

//     let t = true;
//     if t {
//         println!("Success!")
//     }
// } 

// 4.找错误
// fn main() {
//     let f = true;
//     let t = true || false;
//     assert_eq!(t, f);

//     println!("Success!")
// }


// 5.让代码工作，但不要修改 `implicitly_ret_unit` !
// fn main() {
//     let v0: () = ();

//     let _v = (2, 3);
//     assert_eq!(v0, implicitly_ret_unit());

//     println!("Success!")
// }

// fn implicitly_ret_unit() {
//     println!("I will return a ()")
// }


// 6.让代码工作：修改 `assert!` 中的 `4` 
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!")
}
