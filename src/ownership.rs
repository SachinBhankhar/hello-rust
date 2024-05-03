use std::i32;

pub fn ownership() {
    let mut s_vec: Vec<i32> = vec![1, 3, 4, 34, 432, 43];
    s_vec.push(90);
    let slice = &s_vec[0..4];
    println!("{:?}", s_vec);
    println!("{:?}", slice);
    println!("{:?");
}

fn take_ownership(mut s: Vec<i32>) {
    s.insert(0, 9);
}
