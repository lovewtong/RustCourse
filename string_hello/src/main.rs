// 字符串和切片
// fn main() {
//     let my_name = "Pascal";
//     greet(my_name);
// }

// fn greet(name: &str) {
//     println!("Hello, {}!", name);
// }
// 以上错误的原因是greet这个方法，调用的name类型是String，但是my_name的类型默认的是&str

// #![allow(unused)]

fn main() {

    let s = String::from("hello world");
    let _len = s.len();
    //以下两个表达的都是一样的，第一个是切片，第二个是range序列使用
    let hello = &s[0..5];
    let _hello2 = &s[..5];
    
    let world = &s[6..11];

    //想要String的最后一个字节
    let len = s.len();
    let _slice = &s[4..len];
    let _slice: &str = &s[4..];
    
    //中文要注意，一个中文占位3个字节
    let chinese = String::from("你好我是汉字");
    let slice3= &chinese[6..];
    println!("中文字符切片后是,{}",slice3);
    //截取完整的String切片
    let _len = s.len();
    println!("第一个字符切片是,{}",hello);
    println!("第二个字符切片是,{}",world);
    
    let _idan = String::from("नमस्ते");
    // let cutIdan = &idan[4..]; 错误

    //追加(Push)
    let mut s = String::from("Hello,");
    s.push_str("rust");
    println!("追加后的字符{}",s);

    s.push('!');
    println!("追加后的字符{}",s);

    //插入(Inster)
    let mut str2 = String::from("HelloRust");
    str2.insert(5,',');
    println!("插入后的字符{}",str2);

    //替换(Replace)替换后返回一个新值
    let str3 = str2.replace("Rust", "Java");
    let new_string_replace = str3.replace("Rust", "RUST");
    println!("替换后的字符{}",new_string_replace);
    //替换(Replacen)替换一个或者几个后返回一个新值
    let string_replacen = "i like rust,learning rust is my favorite!";
    let new_string_replacen = string_replacen.replacen("rust", "java", 1);
    println!("替换后的字符{}",new_string_replacen);
    //替换(replace_range)替换范围内后的就返回原来已改变的值
    let mut string_replace_range = String::from("i like rust!");
    string_replace_range.replace_range(7..8, "R");
    println!("替换后的字符{}",string_replace_range);

    //删除(Delete)
    //pop--删除并返回最后一个字符，直接操作原来的字符
    let mut string_pop = String::from("rsut pop 中文！");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

    //remove--删除并返回字符串中指定的字符
    //该方法是直接操作原来的字符串。无返回值。该方法 truncate() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
    let mut string_remove = String::from("测试remove方法");
    println!(
        "看看string_remove占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    //删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);

    //truncate--删除字符串中从指定位置开始到结尾的全部字符
    //该方法是直接操作原来的字符串。无返回值。该方法 truncate() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
    let mut string_truncate = String::from("测试truncate方法");
    string_truncate.truncate(3);
    dbg!(string_truncate);

    //clear--清空字符串
    //该方法是直接操作原来的字符串。调用后，删除字符串中的所有字符，相当于 truncate() 方法参数为 0 的时候。
    let mut string_clear = String::from("测试clear方法");
    string_clear.clear();
    dbg!(string_clear);

    //连接 (Concatenate)
    //1、使用 + 或者 += 连接字符串使用 + 或者 += 连接字符串，要求右边的参数必须为字符串的切片引用（Slice）类型。其实当调用 + 的操作符时，
    //相当于调用了 std::string 标准库中的 add() 方法，这里 add() 方法的第二个参数是一个引用的类型。因此我们在使用 +， 必须传递切片引用类型。
    //不能直接传递 String 类型。+ 和 += 都是返回一个新的字符串。所以变量声明可以不需要 mut 关键字修饰。
    let string_appened = String::from("Hello");
    let string_rust = String::from("rust");
    let result = string_appened + &string_rust;
    let mut result = result + "!";
    result += "!!!";

    println!(
        "连接字符串 + -> {}",result
    );

    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3,"hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);

    //format!
    //format! 这种方式适用于 String 和 &str 。format! 的用法与 print! 的用法类似，详见格式化输出。
    let s1 = "hello";
    let s2 = String::from("fromat!");
    let s = format!("{}{}{}!",s1,",",s2);

    dbg!(s);

    //字符串转义
    //通过\ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);


    //UTF-8字符串
    //字符
    for c in "中国人".chars() {
        println!("{}",c);
    }

    //字节
    for b in "中国人".bytes(){
        println!("{}",b)
    }

    //获取子串
    //想要准确的从 UTF-8 字符串中获取子串是较为复杂的事情，例如想要从 holla中国人नमस्ते 这种变长的字符串中取出某一个子串，使用标准库你是做不到的。
    //你需要在 crates.io 上搜索 utf8 来寻找想要的功能。可以考虑尝试下这个库：utf8_slice。
    let s = "holla中国人नमस्ते";
    
    //切割字符串
    let str1 = utf8_slice::slice(s, 0, 5);
    let str2 = utf8_slice::slice(s, 5, 8);
    let str3 = utf8_slice::from(s,8);

    println!("str1{}",str1);
    println!("str2{}",str2);
    println!("str3{}",str3);

    
}

