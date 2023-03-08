// 为什么rust变量要设计成默认情况下是不可变的呢？
// 1.对于大一点的多线程场景来说，同一个名字的变量可能多处引用，其中一部分代码假定该变量的值永远不变，另外一部分确改变了这个值
// 2.在有大量对象的场景，没有可变性，性能会非常低下，内存拷贝的成本很高
fn main() {
    // Rust对常量的命名约定是在单词之间使用全大写加下划线
    // 常量使用const而不是let
    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing隐藏，隐藏可以看作是再次使用let时，就是在使用隐藏
    // 隐藏可以更换值的类型，但是mut不行
    let space = "   ";
    let _space = space.len();
    // mut无法改变变量的类型
    // let mutspace = "   ";
    // let _space = space.len();

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// 变量解构
// fn main() {
//     let (a, mut b): (bool,bool) = (true, false);
//     // a = true,不可变; b = false，可变
//     println!("a = {:?}, b = {:?}", a, b);

//     b = true;
//     assert_eq!(a, b);
// }

// // 解构式赋值
// struct Struct {
//     e: i32
// }

// fn main() {
//     let (a, b, c, d, e);

//     (a, b) = (1, 2);
//     // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
//     [c, .., d, _] = [1, 2, 3, 4, 5];
//     Struct { e, .. } = Struct { e: 5 };

//     assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
// }

// 变量遮蔽(shadowing)
// fn main() {
//     let x = 5;
//     // 在main函数的作用域内对之前的x进行遮蔽
//     let x = x + 1;

//     {
//         // 在当前的花括号作用域内，对之前的x进行遮蔽
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {}", x);
//     }

//     println!("The value of x is: {}", x);
// }
// fn main() {
// // 字符串类型
// let spaces = "   ";
// // usize数值类型
// let spaces = spaces.len();
// }

// test

// 1.修复下面代码的错误并尽可能少的修改

// fn main() {
//     let x: i32; // 未初始化，但被使用
//     let y: i32; // 未初始化，也未被使用
//     println!("x is equal to {}", x);
// }

// 定义x,y。且用_把y变成可忽略的变量
// fn main() {
//     let x= 32_i32; // 未初始化，但被使用
//     let _y= 32_i32; // 未初始化，也未被使用
//     println!("x is equal to {}", x);
// }

// 2.完形填空，让代码编译
// fn main() {
//     let __ = 1;
//     __ += 2;

//     println!("x = {}", x);
// }

// mut将变量设置成可变
// fn main() {
//     let mut x = 1;
//     x += 2;

//     println!("x = {}", x);
// }

// 3.修复下面代码的错误并使用尽可能少的改变
// fn main() {
//     let x: i32 = 10;
//     {
//         let y: i32 = 5;
//         println!("x 的值是 {}, y 的值是 {}", x, y);
//     }
//     println!("x 的值是 {}, y 的值是 {}", x, y);
// }

// 重新定义一个y值
// fn main() {
//     let x: i32 = 10;
//     {
//         let y: i32 = 5;
//         println!("x 的值是 {}, y 的值是 {}", x, y);
//     }
//     let y = 32;
//     println!("x 的值是 {}, y 的值是 {}", x, y);
// }

// 4.修复错误
// fn main() {
//     println!("{}, world", x);
// }

// fn define_x() {
//     let x = "hello";
// }

// 调用函数输出
// fn main() {
//     define_x();
// }

// fn define_x() {
//     let x = "hello";
//     println!("{}, world", x);
// }

// 5.只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 5);
//     }

//     assert_eq!(x, 12);

//     let x = 42;
//     println!("{}", x); // 输出 "42".
// }

// 使其相等
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(5, 5);
//     }

//     assert_eq!(12, 12);

//     let x = 42;
//     println!("{}", x); // 输出 "42".
// }

// 6.删除一行代码以通过编译
// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // 遮蔽且再次绑定
//     let x = x;
//     x += 3;

//     let y = 4;
//     // 遮蔽
//     let y = "I can also be bound to text!";
// }

// 遮蔽过后let x不再是可变的了，是默认不可变的
// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // 遮蔽且再次绑定
//     let x = x;

//     let y = 4;
//     // 遮蔽
//     let y = "I can also be bound to text!";
// }

// 7.使用以下方法来修复编译器输出的 warning
// fn main() {
//     let x = 1;
// }

// compiler warning: unused variable: `x`

// 给let x声明是未使用的变量
// fn main() {
//     let _x = 1;
// }

// 打印输出
// fn main() {
//     let x = 1;
//     println!("x is {}", x)
// }

// 8.修复下面代码的错误并尽可能少的修改
// fn main() {
//     let (x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }

// 将let x视为可变的变量
// fn main() {
//     let (mut x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }

// 遮蔽
// fn main() {
//     let (_x, y) = (1, 2);
//     let x = 1+2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }

// 9.解构式赋值

// fn main() {
//     let (x, y);
//     (x,..) = (3, 4);
//     [.., y] = [1, 2];
//     // 填空，让代码工作
//     assert_eq!([x,y], __);
// }

// //填值
// fn main() {
//     let (x, y);
//     (x,..) = (3, 4);
//     [.., y] = [1, 2];
//     // 填空，让代码工作
//     assert_eq!([x,y], [3,2]);
// }
