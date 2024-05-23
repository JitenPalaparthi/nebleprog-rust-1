
fn main() {
    println!("Hello, world!");

    let s1 = "Hello".to_string();
    // let s2 = s1; // move .. the ownership of "Hello" is moved to s2
    {
        let s2 = &s1; // borrow .. How long ?
        println!("s1={}",s1); 
    }
    //let s3 = &s1;
    // let s4 = &s1;// who is the owner of "Hello" , Within the scope the ownership is temp moved(borrowed)
    // How long ? as long as the borrowd scope is
    println!("s1:{}",s1);

    let mut s1 = "Hello".to_string(); // new variable using shadowing

    {
        let s2: &mut String = &mut s1; // mutable borrow is the borrower can also mutate the value
        s2.push_str(" World");
        // s1.push_str("!");
        // println!("s1:{}",s1);
         println!("s1:{}",s2);
    } 


    s1.push_str("!");
    println!("s1:{}",s1);

    let l = get_length1(&s1); // borrowing the ownership so as soon as the scope in the function ends, the ownership is given back to s1
    println!("s1:{}",s1);

    let l = get_length2(s1); // moving the ownership so no longer s1 is the owner for the value
    println!("s1:{}",s1);

}
// borrowing
// to reference a value without taking the ownership of it

// There are two kinds of borrows. a). mutable borrow b). immutable borrow
// 1. Multiple immutable references: You can have multiple immutable references to a value simultaneously



fn get_length1(s:&String)->i32{
    return s.len() as i32;
}

fn get_length2(s:String)->i32{
    return s.len() as i32;
}