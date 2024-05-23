use std::fmt;
use core::fmt::Display;
fn main() {
    let s1 = Square(25.25);
    let r1 = Rect::new(12.35, 14.56);
    let r2 = Rect::new(12.35, 14.56);
    let r3 = Rect::new(12.35, 14.56);
    let c1 = Cuboid::new(12.34,14.56,10.45);
   
    let mut shapes:Vec<&dyn Area> = vec![&s1,&r1,&c1,&r2,&r3];

    let b1:Box<&dyn Area> = Box::new(&s1); // another example

    for v in shapes{
       let a = v.area_of();
       println!("Area of :{}",a);
    }

}

// trait is similar to interface in other programming languages
// trait enable you to write generic code
// the code can operate on diffent types.

trait Area {
    fn area_of(&self) -> f32;
}

fn area(o: impl Area) {
    println!("Area:{}", o.area_of());
}

struct Square(f32);

impl Area for Square {
    fn area_of(&self) -> f32 {
        return self.0 * self.0;
    }
}

impl Display for Square{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Square Side:{}", self.0)
    }
}

impl Square {
    fn display(&self) {
        println!("I am square and the value i have is {}", self.0);
    }
}

struct Rect {
    l: f32,
    b: f32,
}

impl Area for Rect {
    fn area_of(&self) -> f32 {
        return self.l * self.b;
    }
}

impl Rect {
    fn new(l: f32, b: f32) -> Self {
        Rect { l: l, b: b }
    }
    fn display(&self) {
        println!("Length:{} Bredth:{}", self.l, self.b);
    }
}



struct Cuboid {
    l: f32,
    b: f32,
    h:f32,
}

impl Area for Cuboid {
    fn area_of(&self) -> f32 {
        return self.l * self.b * self.h;
    }
}

impl Cuboid {
    fn new(l: f32, b: f32,h:f32) -> Self {
        Cuboid { l: l, b: b ,h:h}
    }
    fn display(&self) {
        println!("Length:{} Bredth:{} Height:{}", self.l, self.b,self.h);
    }
}
