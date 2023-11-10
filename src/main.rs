mod api;

use api::user::User;

fn main() {
    println!("Hello, world!");
    let u1 = User::new_user(String::from("tom"), 5);
    println!("user name: {}", u1.name());
    println!("1+2: {}", api::user::add(1, 2));
}
