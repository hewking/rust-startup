use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜数字游戏!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("请输入你猜的数字:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("读取行失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜的数字是: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了!"),
            Ordering::Greater => println!("太大了!"),
            Ordering::Equal => {
                println!("你猜对了!");
                break;
            }
        }
    }
}