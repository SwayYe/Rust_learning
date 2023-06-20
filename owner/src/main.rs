// fn main() { // example of scope
//     // s un-usable
//     let s = "hello"; // s usable
//                      // now we can operate on s
// } // the scope of s ends here, s no longer usable

// fn main() {  // example of String stored on a heap
//     let mut s = String::from("Hello");
//     s.push_str(", world");  // add string to s
//     println!("{}", s);
// }


// fn main(){  // example of a failed borrow
//     let s1=String::from("Hello");
//     let s2=s1;  // now s1 has beem removed from attachment to "Hello"
//     println!("{}",s1);  // error because s1 is invalid now
// }


// fn main(){  // example of a clone
//     let s1=String::from("Hello");
//     let s2=s1.clone();  // s1 is not removed, a copy of "Hello" is created
//     println!("{}, {}",s1,s2);  // no error
// }

// fn main(){  // example of copy on stack
//     let x = 5;
//     let y = x;
//     println!("{}, {}", x, y);
// }

// fn main(){
//     let s = String::from("Hello World");
//     take_ownership(s);  //此时s的值被移动到函数里面了，从这之后s就不再有效了
//     let x =5;
//     makes_copy(x);
//     println!("x: {}", x);

//     let s1 = gives_ownership(); //返回值所有权的移动
//     let s2 = String::from("hello");
//     let s3 = takes_and_gives_back(s2); //s2所有权移动到了s3
// }

fn take_ownership(some_string:String){
    println!("{}", some_string)
}   //函数结束时，会调用drop函数，some_string占用的内存被释放


fn makes_copy(some_number:i32){
    println!("{}", some_number);
}   //这里的some_number是源数据的副本，函数结束时，只有副本被释放

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}


fn main(){
    let s1 = String::from("hello");
    let (s2,len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}    

fn calculate_length(s:String) -> (String, usize){
    let length = s.len();
    (s,length)
}