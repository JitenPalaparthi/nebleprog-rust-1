pub mod rectangle{
    pub struct Rectangle{
        pub l:f32,
        pub b:f32,
    }
    impl Rectangle{
        pub fn area(&self)->f32{
            self.l*self.b
        }
    }
}