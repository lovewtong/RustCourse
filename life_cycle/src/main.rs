fn main() {
    
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("the longest str is :{}",result);

}

// 'a一种默认的生命周期注解写法，生命周期并不改变任何引用的生命周期的长短，当指定了泛型生命周期后函数也能接收任何生命周期的引用
// 单个的生命周期并无多大意义，一个函数的参数的联系，所以参数生命周期的注解一致
fn longest<'a>(x: &'a str,y: &'a str) -> &'a str {
    
    if x.len() > y.len() {
        x
    }else {
        y
    }

}

