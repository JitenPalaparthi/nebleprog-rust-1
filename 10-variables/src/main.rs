fn main() {
    let b1: Box<i32> = Box::new(100); // A pointer type that uniquely owns a heap allocation of type i32
    let mut b2 = 100;

    let mut r1: &i32 = &b2;// reference // mut at r1 does not work.
    let r1 = &mut b2;

    *r1 = 200; // dereference

    let r1 = &b2 as *const i32; //immutable raw pointer

    let r2 = &mut b2 as *mut i32; // mutable raw pointer
    
    unsafe {
        *r2 = 300;
    }

    /* psudo c code to explain null pointer dereference
     int n =100;
     *int ptr = &n;
     *ptr = 200; // dereference okay

     *int ptr1;
     *ptr1 = 300; // null pointer dereferencing .Not okay
    */
}
// stack memory
// How to create a variable that stores in heap memory
// there are two kinds of references in rust 1. immutable ref 2. mutable ref
