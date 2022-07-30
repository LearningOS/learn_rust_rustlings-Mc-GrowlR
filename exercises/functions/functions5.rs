// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)



//函数返回值
fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
