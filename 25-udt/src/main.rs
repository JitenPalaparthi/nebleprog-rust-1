fn main() {

    let p1 = Point::new(12.45,16.54);
    let p2 = p1.movep();
    let r1 = Point::newR(12.34, 14.56);
}

struct Point{
    x:f32,
    y:f32,
}

struct Rect{
    x:f32,
    y:f32,
}

impl Point{
    fn new(x:f32,y:f32)->Self{
        Point{x:x,y:y}
    }
    fn newR(x:f32,y:f32)->Rect{
        Rect{x:x,y:y}
    }


    fn movep(&self)->Point{
        Point{x:self.x,y:self.y}
    }

}