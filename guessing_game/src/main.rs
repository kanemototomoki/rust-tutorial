use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("数を当ててごらん");
    println!("予想を入力してね!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("// 秘密の番号はこれ!: {}", secret_number);



    loop {
        let mut guess = String::new();

        io::stdin()
          .read_line(&mut guess)
          .expect("行の読み込みに失敗しました");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}は小さすぎ！", guess),
            Ordering::Greater => println!("{}は大きすぎ！", guess),
            Ordering::Equal => {
                println!("正解！");
                break;
            }
        }
        println!("予想を入力してね!");
    }
}
