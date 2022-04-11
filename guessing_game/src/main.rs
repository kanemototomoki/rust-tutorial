use std::io;
use rand::Rng;

fn main() {
    println!("数を当ててごらん");
    println!("予想を入力してね!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("秘密の番号はこれ!: {}", secret_number);

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("行の読み込みに失敗しました");
    println!("あなたの予想: {}", guess);
}
