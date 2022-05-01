fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    let result: Result<i32, String> = Ok(200);

    // match result {
    //     Ok(code) => println!("code: {}", code),
    //     Err(err) => println!("error: {}", err)
    // }

    /**
     * unwrap_or
     * Ok()だった場合はそのまま展開
     * Err()の場合は与えた引数を返す
     */
    println!("result: {}", result.unwrap_or(-1));
    let result: Result<i32, String> = Err("fatal error".to_string());
    println!("result: {}", result.unwrap_or(-1));

    println!("{}", type_of("aaa"));
}
