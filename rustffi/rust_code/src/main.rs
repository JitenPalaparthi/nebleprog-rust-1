// src/main.rs
#[link(name = "mylib")]
extern "C" {
    fn hello_from_c();
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    unsafe {
        hello_from_c();
        let result = add(5, 3);
        println!("5 + 3 = {}", result);
    }
}