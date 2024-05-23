

#[warn(unused_variables)] // please keep this on embedded implementation
//#![no_std]
fn main() {
    let e1 = Direction::East;
    let e2 = Direction::West;
    let e3= Direction::South;
    let e4 = Direction::North;
    print_direction(e1);
    print_direction(e2);
    print_direction(e3);
    print_direction(e4);
 
}

#[derive(Debug,PartialEq,Eq)] // Debug, PartialEq,Eq all these are automatically implemented on a type if you use derive attribute 
// basic enumration 
#[repr(C)]
enum Direction{
    East,
    West,
    South,
    North
}

// Why do we use enums? 

fn print_direction(e1:Direction){
    println!("{:?}",e1 as u8);
}

