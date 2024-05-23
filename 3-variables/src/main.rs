
fn main() {

    let mut x = 100; // 1. It is stored in stack 2. it holds int data 3. type inference declares this variable as i32
    // 4. x is the owner of value 100

    let x = x+1; // shadowing the original variable x

    {
        let x = x *100;
        println!("{x}");
    }
    println!("{x}");

   // x = 1+1;
    println!("{x}");


    let mut str1 = "hello world"; // &str 
    // str1 = "Hello World";
    let mut str2="hello NobleProg folks";

    println!("{str1},{str2}");
    str2="Hey1 Hello NobleProg minds!";
    str2 ="مرحبا بالعالم";

    println!("{str2}");
    // & -> reference 
    // str --> type of a variable but str it self cannot be a type &str.
    // &str -> also called string slice 
    // it is stack allocated
    // it is immutable
    // Here not because it is not let mut &str ="hellow worlde" it is immutable but in general strings are immutable
    // &str struct
    //  ptr (8 byte)    --> starting address of string literal where it is stored
    //  length (8 bytes,usize)  --> the length of the string. usize is based on machine.On 64bit it is 8 bytes

    // in str1 Length= 44 
// println!("Type of str1:{}",type_name::<typeof(str1)>());

// i8 --> 1 byte 
// 

let char1=  'A'; // 4 bytes u32

println!("{char1}");

let char2:u8 = 65; // 1 byte

// let mut chararr = [65,66,67,68];// ['A','B','C','D']
// chararr[4]=94;

println!("{}",char2 as char);

}



