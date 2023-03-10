struct Rectangle{
    width: u32,
    height: u32,
}

// impl块中所有的所有内容都将与Rectangle类型相关联
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    
    // let width = 20;
    // let height = 30;

    let react = Rectangle {width: 30,height: 30};

    // dbg!(&react);

    println!(
        "the ares is {}",
        // area(width, height)
        react.area()
);

}

//原始版本
// fn area(width:u32 ,height:u32) -> u32{

//     return width * height;

// }

//元组版本
fn area(dimensions: (u32,u32)) -> u32{
    dimensions.0 * dimensions.1
}