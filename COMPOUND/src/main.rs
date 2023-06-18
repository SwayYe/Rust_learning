// fn main(){
//     println!("hello world");
//     another_function(5);
// }


// fn another_function(x: i32) {
//     println!("the value of x is: {}", x);
// }    


fn main(){
    let x = 5;

    let y = {
        let x = 1;
        x + 3
    };

    println!("y = {}", y);

    let x = plus_five(6);
    println!("The value of x is: {}", x);
}

fn plus_five(x: i32) -> i32 {
    x+5
}
    