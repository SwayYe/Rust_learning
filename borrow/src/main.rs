// fn main(){   //引用和借用 例子
//     let mut s1 = String::from("Hello");
//     let len = calculate_length(&mut s1);  // pass reference of s1 to the func
//     println!("The length of '{}' is {}.",s1,len);
// }

// fn calculate_length(s:&mut String) ->usize{
//     s.push_str(", world");
//     s.len()
// }

// fn main(){  //数据竞争 例子
//     let mut s=String::from("Hello");
//     let s1= &mut s;
//     let s2= &mut s;
//     println!("The length of '{}' is {}.",s1,s2);
// }

// fn main(){  //通过创建新的作用域，来允许非同时的创建多个可变引用
//     let mut s = String::from("Hello");
//     {
//         let s1 = &mut s;
//     }
//     let s2 = &mut s;
// }

// fn main(){   // 不可以同时拥有一个可变引用和一个不变的引用
//     let mut s = String::from("Hello");
//     let r1 = &s;
//     let r2 = &s;
//     let s1 = &mut s;
//     println!("{} {} {}",r1,r2,s1);
// }

fn main(){  // 例子：r是一个悬空指针
    let r = dangle();
}

fn dangle() -> &String{
    let s = String::from("hello");
    &s
}
