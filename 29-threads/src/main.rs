use std::thread;

fn main(){
    use std::time::Duration;
    let handle1 =thread::spawn(||{ // spawn is going to create a new system thread
        for i in 1..100{
            println!("2--A second thread value: {}",i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    let handle2 =thread::spawn(||{ // spawn is going to create a new system thread
        for i in 1..100{
            println!("3---A third thread value: {}",i);
            thread::sleep(Duration::from_millis(10));
        }
    });
    // let func1 = ||{ // || closure
    //    // path can be used in the inner scope
    //     for i in 1..100{
    //         println!("A second thread value: {}",i);
    //         thread::sleep(Duration::from_millis(10));
    //     }
    // };

    let k = 100; // 
    let add = |a:i32,b:i32|->i32{
        return a+b+k; // k is outer scope of the close but can take that scope into it
    };

   let a= add(10,20);
   println!("{}",a);

  
    for i in 1..100{
        println!("A main thread value: {}",i);
        thread::sleep(Duration::from_millis(10));
    }
    handle1.join().unwrap();
    handle2.join().unwrap();
}
// closure
// closure is also called as an anonymous function in rust, with out having the 
// name of a function, you can create and execute a function.
