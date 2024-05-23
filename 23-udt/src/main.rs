fn main() {
    println!("Hello, world!");
    let n = Nothing;
    n.greet();

    let r1 = Rect(10.23,14.45);
    let a1 = r1.area();

    println!("Area of tupel Rect r1:{}",a1);
    println!("Area of tupel Rect r1:{:.2}",a1);


    let c1 = Cuboiod{l:12.34,b:43.23,h:43.23};
    let a2=c1.area();

    println!("Area of Cuboid:{:.3}",a2);

    // Heap allocated
    let c2 = Box::new(c1);
    let  a3 =c2.area();
    println!("Area of Cuboid:{:.3}",a3);

    // additional explnation on format and tuple stuff..

    let myself = ("Jiten",39,true);
    let s=format!("My name is {}, age is {} married? {}",myself.0,myself.1,myself.2);
    println!("{}",s);
   // let t =(10,20,"hello");

   //let t1 = (12.34,125.67);
   
}

// Unit struct
// Struct contain members --> fields or methods

struct Nothing; // this is unit struct so does not have any memory upon instatiation of the variable

impl Nothing{
    fn greet(&self){
        println!("Do nothing");
    }
}


// Tuple struct
// A tuple with a name and can also have methods
struct Rect(f32,f32);

impl Rect{
    fn area(&self)->f32{
        let Rect(l,b)=self;
        return l*b;
       // return self.0 *self.1;
    }
}

// struct with fields and methods

struct Cuboiod{
    l:f32,
    b:f32,
    h:f32,
}

impl Cuboiod{
    fn area(&self)->f32{
       return self.l*self.b*self.h;
    }
}


