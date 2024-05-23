use packages::greet;
use packages::shape::{greet_shape};
use packages::shape::rect::rectangle;
use packages::shape::square::Squre;
fn main() {
   greet(); 
   greet_shape();
   let r1 = rectangle::Rectangle{l:12.432,b:34.34};
   let a1 = r1.area();
   println!("Area:{}",a1);

   let s1 = Squre(25.25);

   let a2=s1.area();
   println!("Area:{}",a2);

   let c1 = packages::Cuboid{l:10.2,b:12.3,h:14.3};

}

// lib
// 