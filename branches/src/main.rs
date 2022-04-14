use std::io;

fn main() {
    println!("数字を入力してね！");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("行の読み込みに失敗しました。");

    let input: u32 = input.trim().parse().expect("Invalid value");

    if input % 2 == 0 {
        println!("{}は偶数です。", input);
    } else {
        println!("{}は奇数です。", input);
    }
}
