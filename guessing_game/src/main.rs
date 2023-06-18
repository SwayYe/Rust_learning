use std::io;    
use std::cmp::Ordering;
use rand::Rng;  // prelude  // trait

fn main() {
    println!("猜数游戏！");
    let secret_number = rand::thread_rng().gen_range(1, 101);   // i32 u32 i64    
    // println!("神秘数字是：{}", secret_number);

    loop {
        println!("猜测一个数");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        //shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("你猜测的数是：{}", guess);
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),   // arm
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
