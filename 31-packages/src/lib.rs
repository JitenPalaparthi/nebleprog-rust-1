pub fn greet(){
    println!("Hello World!");
}

fn igreet(){
    println!("Hello World!");
}

pub struct Cuboid{
    pub l:f32,
    pub b:f32,
    pub h:f32,
}

// the Cuboid and greet function can be directly called with the create name in main
// any file with .rs extension is treated as a module
// all the direct files (modules) should be added in lib.rs file as below
// shape.rs and mygreet.rs files are there so add them as mod here in this file
pub mod shape;
pub mod mygreet;

// if there is more code to be organized in directories example shape directory
// all .rs files in shape directory should be added in the shape.rs file which is similar to above
// in shape.rs => pub mod rectangle; pub mod square;