// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

fn main() {
    call_me(4); // 函数指定了参数，必须传
}

fn call_me(num: u32) {
    for i in 0..num { // range
        println!("Ring! Call number {}", i + 1);
    }
}
