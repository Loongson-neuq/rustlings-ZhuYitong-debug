// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
} //Rust 使用字符串传入参数时，如果不是需要修改传入的 String，就尽量设置参数类型为 &str，其中&String 会被自动解引用成&str

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
