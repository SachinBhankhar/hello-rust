pub fn loops() {
    let mut i = 1;
    let arr = [1,2,3,4,5];
    let result = loop {
        i += 1;
        if i > 5 {
            break i;
        }
    };
    println!("{result}");
    for num in arr {
        println!("{num}");
    }
}
