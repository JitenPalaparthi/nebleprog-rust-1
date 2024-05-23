pub struct Squre(pub f32);

impl Squre{
    pub fn area(&self)->f32{
        self.0*self.0
    }
}