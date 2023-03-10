fn main() {
    // let number = 3;

    // if number < 5 {
    //     println!("number is small");
    // }else {
    //     println!("number is big")
    // }

    // if与else的输出类型必须一致，不一致将会报错
    let boolean = true;

    let number = if boolean{
        5
    } else{
        6
    };

    println!("the number is {}",number)
}
