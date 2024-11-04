// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned()); //用于将字符串切片或其他类型的引用转换为拥有所有权的类型
    string("nice weather".into()); //将一个值转换为另一个类型。into 方法通常会消耗调用它的值，因为它会转移所有权。
                                       //into不是通常，只有在实现了复制语义的情况下才是复制值而不消耗原来的所有权。into 本身就是按值拷贝传入参数的
    string(format!("Interpolation {}", "Station")); //format!返回的是一个 String 类型，而不是字符串切片。format!会创建一个新的字符串，并将格式化后的结果存储在String中。
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); //用于将字符串中的所有字符转换为小写字母
}
