// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let vec = foo();

    println!("{} has length {} content `{:?}`", "vec", vec.len(), vec);

    let mut vec = vec;

    vec.push(88);

    println!("{} has length {} content `{:?}`", "vec", vec.len(), vec);
}

// `foo()` does NOT take `vec: Vec<i32>` as argument
fn foo() -> Vec<i32> {
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}