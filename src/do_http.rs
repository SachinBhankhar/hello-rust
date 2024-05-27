use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    userId: i32,
    id: i32,
    title: String,
    completed: bool,
}

pub async fn do_http() -> Vec<User> {
    let res = reqwest::get("https://jsonplaceholder.typicode.com/todos/")
        .await
        .unwrap()
        .text()
        .await;

    let body = res.expect("{}");

    let users: Vec<User> = serde_json::from_str(&body).expect("error");
    users
}
pub async fn do_http_one(id: &str) -> User {
    let res = reqwest::get(format!("https://jsonplaceholder.typicode.com/todos/{}", id))
        .await
        .unwrap()
        .text()
        .await;

    let body = res.expect("{}");
    println!("{body}");

    let user: User = serde_json::from_str(&body).expect("error");
    user
}
