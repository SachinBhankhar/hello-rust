struct User {
    name: String,
    age: i32,
}
impl User {
    fn is_adult(self) -> bool {
        self.age >= 18
    }
}
pub fn learn_struct() {
    let user = User {
        name: String::from("hello"),
        age: 12,
    };
    println!("{}", user.name);
    println!("{}", user.age);
    println!("{}", user.is_adult());
}
