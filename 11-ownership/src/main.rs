fn main() {

// stack allocated

let mut x1 = 100;

let mut y1 = x1; // copy 

x1+=1;

println!("x1:{}",x1);

println!("y1:{}",y1);

// heap allocated

let mut x2 = Box::new(100);

let mut y2 = x2;//  the ownership is moved

*y2 = 200;

//println!("x2:{}",x2); // cant use x2 becase x2 is invalidated
println!("y2:{}",y2);


let mut s1:String = String::from("Hello World"); // &str is stack allocated String is heap allocated.
s1.push_str("! How are you doing?"); // append the string
// String structure 
// Ptr
// Length
// Capacity
{
let mut s2: String = s1; // value moved from s1 to s2 and the ownership as well
s2.push_str("Okay");
s1=s2;
println!("s1:{}",s1);
}

println!("s1:{}",s1);
// Who is the owner of "Hello World! How are you doing?"

let x1 = 25;
let sq = get_square(x1);
println!("square of x1:{}",sq);
println!("x1:{}",x1);


// let l = get_length1(s1);
// println!("lenth of s1:{}",l);
// println!("s1:{}",s1);
let l:i32;
(l,s1) = get_length2(s1);
println!("s1:{}",s1);

}

// Ownership:
// Ownershp ensure the memory safty and avoid data rases in rust. It is an essential concept in rust.
// Ownership rules in rust
// 1. Each value in rust has a single owner. A value can have only one owner at a time
// 2. when owner goes out of the scope the value is dropped.
// 3. When the owner goes out of the scope the memory is deallocated

fn get_length1(s:String)->i32{
    s.len() as i32
    // s is deallocated here
}

fn get_length2(s:String)->(i32,String){
   return ( s.len() as i32,s);
    // s is deallocated here
}

fn get_length3(s1:String,s2:String)->i32{

    (s1.len()+s2.len()) as i32
}

fn get_square(x:i32)->i32{
    x *x
}