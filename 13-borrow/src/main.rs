fn main() {
   
let mut x1 = 100;

// let y1 = &x1; // borrow 
// let z1 = &x1; 
{
let mut y1 = &mut x1;
//let z1 = &x1;
*y1 = 200; // dereference or mutate a reference
}


{
    let mut z1 = &mut x1;
    //let z1 = &x1;
    *z1 = 300; // dereference or mutate a reference
    }


    let mut s1 = "Hello".to_string();

    append_str(&mut s1, " World");

    println!("{}",s1);

    let mut x  = 100;

    square(&mut x);

    println!("x:{}",x);

    square(&mut x);

    println!("x:{}",x);

    let s1 = "hello World";
    let s2 = String::from("Hello World!");

    let l1 = get_length1(s1);

    let l2 = get_length1(&s2);

    // let s1 = "hello World";
    // let s2 = String::from("Hello World!");

    // let l1 = get_length2(s1); 

    // let l2 = get_length2(s2);

}   

// borrows
// mutable borrow rules:
// There should only be single mutable reference in a given time
// in a scope, if there are no mutable references then there can be any number of immutable references.
// in a scope, if tehre is a mutable reference then there should be only one referece

// simple task 
// append_string
// take string as a argument
// append more to the string
// do not return the string 
// but in the main after call the function print the string.It should print the addped string
// s1 = "Hello"
// append_str(s1) // append s1 with world
// print(s1) "Hello World"

fn append_str(s:&mut String,str1:&str){
    s.push_str(str1);
}

fn square(num:&mut i32){
        *num *= *num;
}

// &str --> 

fn get_length1(s:&str)->usize{ // &str string slice 
    s.len()
}

fn get_length2(s:String)->usize{ // &str string slice 
    s.len()
}

fn change_str(s:&mut str){
    s.to_string().push_str("hello");
}