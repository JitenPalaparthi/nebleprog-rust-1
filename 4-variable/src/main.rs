fn main() {

    let tuple1 = (39,"Jiten");
    let tuple2 = (39,"Jiten",true);
    let tuple3 = ('A',false,"Hello world",3.14,34234234);

   println!("Age:{}",tuple1.0);
   println!("{:?}",tuple2); // println! is a formatted print and a  friendly print to the developer
   println!("{:#?}",tuple3); //{:?} the debug representation of a value
   // loggers , same logger can be used to print on stdout and also used to store in a log file or bla bla bla

   let (a,b,c)=tuple2;

   println!("Age:{a} Name:{b} Married:{c}");

   let char4 = 'A' as u8;
   let char5 = 69 as u8;

   let (_,name,_)=tuple2;
   // _ blank placeholder
   // 
   println!("Name:{}",name);

}
// tuple: fixed size collection of heterogeneous elements
// in rust any thing that has to be complaint to println! that type has to implement display trait


