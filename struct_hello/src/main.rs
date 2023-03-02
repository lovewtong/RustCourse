// 结构体语法
// fn main() {
//     // 初始化实例时，每个字段都需要初始化
//     // 初始化的字段不需要跟结构体的字段顺序一致
//     // Rust不支持吧结构体的某个字段变成可变的(mut)
//     let mut user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("user1"),
//         active: true,
//         sing_in_count: 1,
//     };

//     user1.email = String::from("someones@example.com");

//     let _user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };
//     //  结构体更新语法跟赋值语句 = 非常相像，因此在上面代码中，user1 的部分字段所有权被转移到 user2 中：username 字段发生了所有权转移，作为结果，user1 无法再被使用。

//     // 聪明的读者肯定要发问了：明明有三个字段进行了自动赋值，为何只有 username 发生了所有权转移？

//     // 仔细回想一下所有权那一节的内容，我们提到了 Copy 特征：实现了 Copy 特征的类型无需所有权转移，可以直接在赋值时进行 数据拷贝，其中 bool 和 u64 类型就实现了 Copy 特征，因此 active 和 sign_in_count 字段在赋值给 user2 时，仅仅发生了拷贝，而不是所有权转移。

//     // 值得注意的是：username 所有权被转移给了 user2，导致了 user1 无法再被使用，但是并不代表 user1 内部的其它字段不能被继续使用，例如：

    
//     println!("{}", user1.active);
//     // 下面这行会报错
//     //println!("{:?}", user1);

// }

// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sing_in_count: u64,
// }

// fn _bulid_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sing_in_count: 1,
//     }
// }


// 结构体的内存排列
// 把结构体中具有所有权的字段转移出去后，将无法再访问该字段，但是可以正常访问其它的字段。
#[derive(Debug)]
 struct File {
   name: String,
   data: Vec<u8>,
 }

 fn main() {
   let f1 = File {
     name: String::from("f1.txt"),
     data: Vec::new(),
   };

   let f1_name = &f1.name;
   let f1_length = &f1.data.len();

   println!("{:?}", f1);
   println!("{} is {} bytes long", f1_name, f1_length);
 }

// 元组结构体
// 元组结构体在你希望有一个整体名称，但是又不关心里面字段的名称时将非常有用。
// 例如上面的 Point 元组结构体，众所周知 3D 点是 (x, y, z) 形式的坐标点，因此我们无需再为内部的字段逐一命名为：x, y, z。
// #![allow(unused)]
// fn main() {
//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);

//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);

//     println!("Color1 is {}",black.0);
// }
