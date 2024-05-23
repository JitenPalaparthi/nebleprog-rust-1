fn main() {
  let mut r1 = Rect::new(10.1,13.4);
  area(&mut r1);
  r1.display();
}

trait Area {
    fn area_of(&mut self) -> f32;
}

fn area(o: &mut impl Area) {
    println!("Area:{}", o.area_of());
}

struct Rect {
    l: f32,
    b: f32,
    a: f32,
}

impl Area for Rect {
    fn area_of(&mut self) -> f32 {
        self.a= self.l * self.b;
        return self.a;
    }
}

impl Rect {
    fn new(l: f32, b: f32) -> Self {
        Rect { l: l, b: b,a:0.0 }
    }
    fn display(&self) {
        println!("Length:{} Bredth:{} Area:{}", self.l, self.b,self.a);
    }
}



// impl --> is a static dispatch
// dyn --> is a dynamic dispatch