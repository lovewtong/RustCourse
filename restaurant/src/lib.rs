pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {

    // 绝对路径
    create::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

fn server_order(){

    fn fix_incorrect_order{
        cook_order();
        super::server_order;
    }

    fn cook_order(){
        
    }

}


