// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut answer = square(3);
    println!("The square of 3 is {}", answer);
}

// 函数返回时不需要加 ";"
fn square(num: i32) -> i32 {
    num * num
}
