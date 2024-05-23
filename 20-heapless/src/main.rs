use heapless::Vec;

fn main() {
    // on the stack
    let mut xs: Vec<u8, 8> = Vec::new(); // can hold up to 8 elements
    xs.push(42).unwrap();
    assert_eq!(xs.pop(), Some(42));
    assert_eq!(xs.pop(),None);
    let r1: Result<(), u8>= xs.push(45);
    // let r2:Result<(),u8>;
   // let mut _v1: std::prelude::v1::Vec<i32>= vec![10,20];
}

// Result  -> enum
// Ok(T)
// Err(R)