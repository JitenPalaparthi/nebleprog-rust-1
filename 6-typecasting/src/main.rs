
//#![no_std] // this is file level and global attribute
#[allow(unconditional_panic)] // this is functional level attribute here.
fn main() {
   
let x1:u32 = 12312;

let y1:i32 = x1 as i32;

println!("{} {}",x1,y1);

let x2 =0b1100; // can directly give binary values appending with 0b

let x3 = 0x65f; // can directly assign hexa values by appending with 0x


let x4:u8 = x1 as u8;
// 1 byte = 4 bytes are shrunk to 1 byte

//let x5= 0b00011000;

println!("{x4}");
println!("{:x}",69420);

//let s1 = "hgello World";
//let x6 = s1 as i32;

let char1 = 'A';

let x6: u64 = (char1 as u8) as u64;

let ok1= false;

let x7 = ok1 as u8;

println!("Bool to int: {}",x7);

let pi1 = 3.14515;

let x8:i32 = pi1 as i32;

let str1 = "45";
let str2 = "3.14";
let str3="ab3.14c";

let x9 = str1.parse::<i32>();
let x10= str2.parse::<f64>();
let x11= str3.parse::<f64>();

println!("str1 to i32:{:#?}",x9);
println!("str2 to f64:{:#?}",x10);
println!("str3 to f64:{:#?}",x11);

let mut x = 1;

x -=1;

let y= 100;

println!("{}",y/x);
}

// primitive type casting
// pointer type casting
// From and Into Traits type casting