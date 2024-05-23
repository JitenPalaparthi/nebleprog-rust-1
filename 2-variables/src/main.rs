static GLOVAL_VAR1 :i32 =10000; // immutable global variable 
static mut GLOBAL_VAR2:i64 =10000; // datasegment

fn main() {

    let num1:i8 = 100;// i8 is created

    let num2 = 101; // i32 is created. type inference, based on the value automatically a type is inffered to a variable

    let ok:bool = false;

    let pi = 3.14;

    let name = "NobleProg"; // assign a string literal , this is called &str

    println!("Num1: {}",num1);
    println!("Num2: {num2}");
    println!("{} {} {} {} {}",num1,num2,pi,ok,name);


    //num1 = 115; //this does not work since mutate num1 to 115

    let mut num3 = 500; // only to mutable varialbles can assign a value more than once

    num3 = num3+1; 
    // ++ ,--  no unary operators

    // let num4:i64; // this is invalid, unless a value is given there is no point of creating a variable in rust
    // println!("{}",num4);

    let num4:i64;

    num4 = 1231*num1 as i64;

    println!("{}",num4);

    const PI:f32 = 3.14; // code segment

    println!("const pi: {PI}");

    unsafe{ // by using this block , you are saying that the block is not compatiable with rust safey guarentees

    GLOBAL_VAR2 = GLOBAL_VAR2+1; // mutating the global variable 

    println!("Mutable Global Variable:{GLOBAL_VAR2}");

    }

}


// mutable and immutable
// primitive datatypes in rust 
// numnbers
// i8,i16,i32,i64,size,usize,u8,u16,u32,u64
// f32,f64
// bool
// char -- unicode characters utf-8, 4 bytes
// &str string slice

// by default every variable in rust is immutable
// 
