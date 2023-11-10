#[derive(Debug)]
pub struct User {
    name: String,
    age: i32
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


}
