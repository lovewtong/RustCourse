use rand::Rng; //Rng 是一个 trait，它定义了随机数生成器应实现的方法,想使用这些方法的话，此 trait 必须在作用域中。
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    //  我们调用rand::thread_rng 函数来获取我们接
    // 下来要使用的随机数生成器：它位于当前执行线程的本地环境中，并从操作系统获取 seed。接下
    // 来，调用随机数生成器的gen_range方法。这个方法由刚才引入到作用域的Rng trait 定义。
    // gen_range 方法接受一个范围表达式作为参数，并在该范围内生成一个随机数。我们在这里使用的
    // 范围表达式采用start..end 形式，它包含下限但不包含上限，所以需要指定1..101 来请求一个1
    // 和100之间的数.另外，我们也可以传递1..=100 ，这是等价的。
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("The secret number is {}", secret_number);

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // secert_number默认是数字类型，我们将guess转为数字类型:i34,u32都可以
        // 这里的新guess之所以可以继续使用，是因为rust是可以允许隐藏旧值的，所以旧guess被showing了
        // 调用str.trim()方法是为了去除掉\r或者\r\n的，因为用户输入的guess不是原本的样子，用户输入后也要继续输入一个enter键才能执行
        // str的.parse()方法是将字符解析为数字，这个方法可以解析多种字符的，因此需要指定一个类型，但是parse()方法只能作用于可以逻辑转化为数字的字符，所以我们需要一个Result类型替我们处理潜在的错误
        // 将except换为match，从遇到错误就崩溃到处理错误 
        // let guess: i32 = guess.trim().parse().expect("please type a number");
        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guessed:{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
