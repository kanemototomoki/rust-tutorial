fn main() {

    let arr = [10, 20, 30, 40, 50];

    for value in arr.iter() {
        println!("{}", value);
    }

    println!("-----");

    for num in (1..6).rev() {
        println!("{}", num);
    }
}
