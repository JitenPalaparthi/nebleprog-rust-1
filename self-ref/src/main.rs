use std::thread;
use std::time::Duration;

use udt::shape::{greet};
use udt::shape::rect::greet3 as gt3;
use udt::greet as gt;
fn main() { 
    greet();
    //use udt::greet as gt;
    gt();
    gt3();

   let handle1= thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned--1 thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    let handle2= thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned----2 thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// Constructor --? 

