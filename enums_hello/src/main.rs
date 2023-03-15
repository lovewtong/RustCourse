fn main() {

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    
    let some_number = Some(5);
    let some_string = Some("Hello,enums!");

    let absent_number:Option<i32> = Option::None;

    let x:i32 = 5;
    let y = Some(10);

    let sum = x + y;

    println!("sum is :{}",sum);
}
//文章 作者名 日期 类别 标签

// 定义一个函数来获取任何IpAddrKind
fn route(ip_type: IpAddrKind) {
    
}
// 枚举类型，V4和V6都是属于IpAddrKind的
enum IpAddrKind {
    V4,
    V6,
}

enum Option<T> {
    Some(T),
    None,
}