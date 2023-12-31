#[derive(Debug)]
pub struct User {
    name: String,
    age: i32
}

// 定义一个新的用户
#[derive(Debug)]
struct MyUser {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    pub fn new_user(name: String, age: i32) -> User {
        User{
            name,
            age
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}


#[cfg(test)]
mod tests {
    use crate::api::user::MyUser;

    #[test]
    fn it_works() {
        println!("Hello, mytest!");
        assert_eq!(2 + 2, 4);
    }

    #[test]
    // 切片想要包含 String 的最后一个字节
    fn slice_last() {
        let s = String::from("hello");

        let slice = &s[0..2];
        println!("slice {}", slice);

        let slice = &s[..2];
        println!("slice {}", slice);
    }

    #[test]
    // 截取完整的 String 切片
    fn slice_all() {
        let s = String::from("hello");

        let len = s.len();

        println!("len {}", len);

        let slice = &s[0..len];
        let slice = &s[..];
        println!("slice {}", slice);
    }

    #[test]
    // 截取完整的 String 切片
    fn slice_index_error() {
        let s = "中国人";
        let a = &s[0..2];
        //  println!("{}",a);
    }

    #[test]
    // 截取完整的 String 切片
    fn slice_mut() {
        let mut s = String::from("hello world");

        let word = first_word(&s);

        s.clear(); // error!

        // 请注意：假如去掉注解，则编译失败，\
        // println!("the first word is: {}", word);
    }

    fn first_word(s: &String) -> &str {
        &s[..1]
    }

    #[test]
    // 截取完整的 String 切片
    fn slice_other() {
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);
    }

    #[test]
    // string 切片的 push 方法 字符串变量必须由 mut 关键字修饰。
    fn str_push() {
        let mut s = String::from("Hello ");
        s.push_str("rust");
        println!("追加字符串 push_str() -> {}", s);

        s.push('!');
        println!("追加字符 push() -> {}", s);
    }

    #[test]
    // string 切片的 push 方法 字符串变量必须由 mut 关键字修饰。
    fn str_replace() {
        let string_replace = String::from("I like rust. Learning rust is my favorite!");
        let new_string_replace = string_replace.replace("rust", "RUST");
        println!("追加字符 push() -> {}", new_string_replace);
        // The code also uses the dbg!() macro to print the value of variables to the console, which can be helpful for debugging purposes.
        dbg!(new_string_replace);
    }

    #[test]
    // string 切片的 push 方法 字符串变量必须由 mut 关键字修饰。
    fn str_delete() {
        let mut string_clear = String::from("string clear");
        string_clear.clear();
        dbg!(string_clear);
    }

    #[test]
    // string 切片的 push 方法 字符串变量必须由 mut 关键字修饰。
    fn tuple_test() {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is {}.", s2, len);
    }

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() 返回字符串的长度
        (s, length)
    }

    #[test]
    // username 所有权被转移给了 user2，导致了 user1 无法再被使用，但是并不代表 user1 内部的其它字段不能被继续使用，例如：
    fn user_test() {
        let user1 = MyUser {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        let user2 = MyUser {
            active: user1.active,
            username: user1.username,
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count,
        };
        println!("{}", user1.active);
        //下面这行会报错
      // println!("{:?}", user1);
    }


}
