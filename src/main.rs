use std::{io, usize};
//mod loops;
//mod ownership;
mod do_http;
mod learn_struct;

#[tokio::main]
async fn main() {
    //tuple
    // let tup: (i32, f32, &str) = (3, 4.2, "string");
    // println!("{:?} {:?} {:?}", tup.0, tup.1, tup.2);

    //arrays
    // let arr = ["sk", "dd"];
    // println!("{:?}", arr);
    // let mut guess = String::new();
    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("enter number only");
    // let index: usize = guess.trim().parse().expect("wrong input");
    // let item = arr[index];
    // println!("{:?}", item);

    //functions
    // let x = plus_one(5);
    // println!("{x}");
    //loops::loops();
    //    ownership::ownership();
    //    learn_struct::learn_struct();
    let id = std::env::args().nth(1).expect("Enter id");
    let users =  do_http::do_http_one(&id).await;
    println!("{:?}",users);
}

//fn plus_one(x: i32) -> i32 {
//    x + 1
//}
