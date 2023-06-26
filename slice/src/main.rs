// fn main() {  // 引子：为什么需要字符串切片
//     let mut s = String::from("Hello world");
//     let wordIndex = first_world(&s);    // wordIndex指向字符串的第一个空格，也就是5
//     s.clear();  //执行完这句话之后，wordIndex就没有意义了
//     println!("{}", wordIndex);
// }

// fn first_world(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate(){
//         if item == b' '{
//             return i;
//         }
//     }
//     s.len()
// }

// fn main(){      // 例子 字符串切片
//     let s = String::from("Hello world");

//     let hello = &s[0..5];   //切片，包含索引0，不包括索引5
//     let world = &s[6..11];

//     // 另一种写法
//     let hello1 = &s[..5];
//     let world1 = &s[6..];

//     let whole = &s[..];
//     println!("{}, {}", hello, world);
// }

// fn main() {  // 使用切片重写最开始的例子
//     let mut s = String::from("Hello world");
//     let word = first_world(&s);   
//     println!("{}", word);
// }

// fn first_world(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate(){
//         if item == b' '{
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

// fn main() {  // 例子：将字符串切片作为参数传递
//     let my_string = String::from("Hello world");
//     let wordIndex = first_world(&my_string[..]);   
//     println!("{}", wordIndex);

//     let my_string_literal = "hello world";
//     let wordIndex = first_world(my_string_literal);
// }

// fn first_world(s: &str) -> &str {    //将字符串切片作为参数传递
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate(){
//         if item == b' '{
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

fn main(){  // 其他切片例子
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}
