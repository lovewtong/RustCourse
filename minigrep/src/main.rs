use std::env;
use std::fs;
use std::process;
use std::error::Error;

struct Config {
    query: String,
    filename: String,
}

// 考虑错误处理以及对参数的合法性进行检查
impl Config {
    fn new(args:&[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query , filename })
    }
}

fn run(config: Config) -> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(());
}

fn main() {
    
    // 读取参数
    let args: Vec<String> = env::args().collect();
    
    // // 参数保存至变量,斤
    // let query = &args[1];
    // let filename = &args[2];

    // println!("Searching for {}", query);
    // println!("In file {}", filename);

    // 非零的退出状态是一个惯例信号，用来告诉调用的进程：该程序以错误状态推出了
    // unwrap_or_else方法：当result是OK时，返回OK内部封装的值；当result是ERR时，会接受一个闭包，这个闭包会在unwrap方法遇到错误时被调用
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments:{}",err);
        process::exit(1);
    });

    println!("Searching for {}",config.query);
    println!("In file {}",config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    run(config);
    // // 读取文件，从main中提取逻辑
    // let contents = fs::read_to_string(config.filename)
    //     .expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);
    // println!("{:?}", args);



    // 提取参数解析器,config名字表面它将返回一个Config实例
    // string.clone虽然可以直接完成数据的完整拷贝，但会消耗更多的时间与内存:因为它需要在堆上分配新的内存空间，不过这样可以使得代码更加整洁
    // 且无需管理引用的生命周期与借用规则，某些情况下，牺牲一小部分的性能来换取代码的简洁性是值得的
    // fn parse_config(args: &[String]) -> Config {
    //     let query = args[1].clone();
    //     let filename = args[2].clone(); 
        
    //     Config {query,filename }
    // }

    // impl Config {
    //     fn new(args:&[String]) -> Config {
    //         let query = args[1].clone();
    //         let filename = args[2].clone();

    //         Config { query , filename }
    //     }
    // }

        
    }
