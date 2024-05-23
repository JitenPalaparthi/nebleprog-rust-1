//#![no_std]
fn main() {

    let number1:Option<i32>= Some(100);
    let number2:Option<i32>=None;
    // Either there is some value Some(100)
    // or None
   // println!("{number1:?}");

    // let number1:i32;
    // let ok:bool;
    // let name:&str;

    match number1{
        Some(n)=>println!("{n}"),
        None=>println!("None value, or no value")
    }


    match number2{
        Some(n)=>println!("{n}"),
        None=>println!("None value, or no value")
    }

    let str1:&str = "3.14";
    let str2="a3.14";

    let x1: Result<f64, std::num::ParseFloatError> = str1.parse::<f64>();
    let x2: Result<f32, std::num::ParseFloatError> =str2.parse::<f32>();

    match x1{
        Ok(n)=>println!("{n}"),
        Err(e)=>println!("{}",e.to_string()),
    }

    match x2{
        Ok(n)=>println!("{n}"),
        Err(e)=>println!("{}",e.to_string()),
    }


}
// there is no null or nil in rust
// variable must have a value 
// value is not automatically type inffered


fn getVal(x:i32)->Result<i32,String>{
    if x>=10{
        return Ok(x);
    }else{
        
        return Err(String::from("low x value"));
    }
}