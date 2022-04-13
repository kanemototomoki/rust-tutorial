fn main() {
    let tup = (
        ("i8", i8::min_value(), i8::max_value()),
        ("i16", i16::min_value(), i16::max_value()),
        ("i32", i32::min_value(), i32::max_value()),
        ("i64", i64::min_value(), i64::max_value()),
        ("i128", i128::min_value(), i128::max_value()),
        ("isize", isize::min_value(), isize::max_value()),
        ("u8", u8::min_value(), u8::max_value()),
        ("u16", u16::min_value(), u16::max_value()),
        ("u32", u32::min_value(), u32::max_value()),
        ("u64", u64::min_value(), u64::max_value()),
        ("u128", u128::min_value(), u128::max_value()),
        ("usize", usize::min_value(), usize::max_value()),
    );

    // println!("{:#?}", tup);
    let x = 5;

    let y = {
        let x = 3;
        x + 1 // 式なのでセミコロン付けない！付けると文になる
    };

    println!("The value of y is: {}", y); // 4

    let x = five();
    println!("{}", x)
}

fn five() -> i32 {
    5
}
