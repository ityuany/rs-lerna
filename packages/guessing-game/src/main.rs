use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("================猜数字游戏!================");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("自动生成的数字是:{}",secret_number);

    loop {
        println!("请输入一个数字:");
        // mut 表示这个变量是  mutable 的 ， 后续的逻辑中是可以改变他的
        let mut guess = String::new();
        // 使用 &mut guess 明确的表示传入了一个可变的变量
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}不能被正确的解析为一个合法的数字,请输入一个正确的数字!",guess);
                continue;
            }
        };
        match guess.cmp(&secret_number){
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("你赢了");
                break;
            }
        }
    }

    println!("================猜数字游戏!================");


  

  
}
