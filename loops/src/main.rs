fn main() {

    // 循环
    // loop {
    //     println!("again!");
    // }

    let mut count = 0;

    'counting_up: loop{
        println!("count = {}",count);
        let mut remaining = 10;

        loop{
            println!("remaining is {}",remaining);
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count is {}",count);

    let a = [10,20,30,40,50];

    for element in a.iter() {
        println!("the value is {}",element);
    }

    //for循环，倒计时，rev()用于反转
    for number in (1..4).rev() {
        println!("{}!",number);
    }
    println!("LIFTOFF!");
}
