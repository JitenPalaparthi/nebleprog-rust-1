fn main() {
    let mut k1 = Box::new(100);
    let k= move ||->Box<i32>{
        println!("Hello:{}",k1);
        return k1;
    };

    k1 = k(); // move
    println!("{}",k1); // k1 is no longer valid

}
