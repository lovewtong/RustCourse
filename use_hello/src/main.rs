use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("123", 123);
}

// use std::fmt::Result;
// use std::io::Result as NewResult;
// fn function1() -> Result {}

// fn function2() -> NewResult<()> {}

// use std::fmt; 
// use std::io;
// fn function1() -> fmt::Result { 
// // --snip--
// }
// fn function2() -> io::Result<()> { 
// // --snip--
// }