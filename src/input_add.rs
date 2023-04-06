use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::io;

pub fn sayHello() {
    let stdout = stdout();
    let message = String::from("Hello fellow my binary.ts!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

pub fn inputAdd(){
    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin().read_line(&mut input1).unwrap();
    io::stdin().read_line(&mut input2).unwrap();

    let num1 = input1.trim().parse::<i32>().unwrap();
    let num2 = input2.trim().parse::<i32>().unwrap();

    println!("{} + {} = {}", num1, num2, num1 + num2);
}