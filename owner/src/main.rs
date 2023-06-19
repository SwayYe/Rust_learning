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

fn main(){  // example of copy on stack
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);
}