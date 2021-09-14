// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)


fn main() {
    call_me(3);
}

fn call_me(num: i32) { // 形参必须指定类型
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
