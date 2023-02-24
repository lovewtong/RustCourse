// 字符串和切片
// fn main() {
//     let my_name = "Pascal";
//     greet(my_name);
// }

// fn greet(name: &str) {
//     println!("Hello, {}!", name);
// }
// 以上错误的原因是greet这个方法，调用的name类型是String，但是my_name的类型默认的是&str

#![allow(unused)]
fn main() {

    let s = String::from("hello world");
    let len = s.len();
    //以下两个表达的都是一样的，第一个是切片，第二个是range序列使用
    let hello = &s[0..5];
    let hello2 = &s[..5];
    
    let world = &s[6..11];

    //想要String的最后一个字节
    let len = s.len();
    let slice = &s[4..len];
    let slice: &str = &s[4..];
    
    //中文要注意，一个中文占位3个字节
    let chinese = String::from("你好我是汉字");
    let slice3= &chinese[6..];
    println!("中文字符切片后是,{}",slice3);
    //截取完整的String切片
    let len = s.len();
    println!("第一个字符切片是,{}",hello);
    println!("第二个字符切片是,{}",world);

}
