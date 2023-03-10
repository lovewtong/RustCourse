// fn main() {                  //s没被创造，所以是无效的
//     let s = "hello";   //s被创造了
// }                            //s离开了作用域，无效

// fn  main() {
//     let mut s = String::from("hello");
//     s.push_str(",world");
//     println!("{}",s);
// }

// fn main() {
//     let s1 = String::from("hello"); // 创建一个 String
//     let len = calculate_length(&s1); // 传入一个引用
//     println!("The length of '{}' is {}.", s1, len); // 输出结果
// }

// fn calculate_length(s: &String) -> usize { // 函数使用引用作为参数
//     s.len() // 返回值为字符串的长度
// }

fn main() {
    
    // let reference_to_nothing = dangle();

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let my_string = String::from("hello world");
    // first_word 中传入    `String` 的    slice
    let word = first_word(&my_string[..]);
    println!("the word is {}",word);
    let my_string_literal = "hello world";

    // first_word 中传入字符串字面值的    slice
    let word = first_word(&my_string_literal[..]);
    println!("the word is {}",word);
    // 因为字符串字面值    **就是** 字符串    slice，
    // 这样写也可以，即不使用    slice 语法！
    let word = first_word(my_string_literal);
    println!("the word is {}",word);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn dangle() -> & String {
//     let s = String::from("hello");

//     &s
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
