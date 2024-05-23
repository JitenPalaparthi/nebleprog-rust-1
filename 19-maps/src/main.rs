use std::collections::HashMap;
// the alternative implementation is heapless crate
fn main() {
    let mut mymap1: HashMap<i32, i32> = HashMap::new();
    mymap1.insert(1, 1001);
    mymap1.insert(2, 2002);

    let mut mymap2: HashMap<i32, i32> = HashMap::with_capacity(4);
    mymap2.insert(1, 1001);
    mymap2.insert(2, 2002);

    let mymap3 = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);

    for (k, v) in &mut mymap1 {
        // iteration
        //*v = 10000;
        println!("Key:{} Value:{}", *k, *v);
    }

    let k = 1;
    let opt: Option<i32> = mymap1.remove(&k);

    match opt {
        Some(v) => {
            println!("Map with the value is deleted :{v}")
        }
        None => {
            println!("no key found")
        }
    }

    /*
    k1:= 100
    var ptr1 *int = &k // ptr1 is a pointer, this holds the address of k

    var ptr2 *int --> what is the value of ptr2 ? nil

    ptr3:=new(int) --> it allocates some memory for int and assigns a value 0 and returns the address of that memory to ptr3
    ptr3 --> it holds the address of value 0
    *ptr3 --> 0
    *ptr3 = 100 // dereference with 100

    let mut k2:&i32 = &100;
    let ptr5= &mut k2;

    let ptr4:&i32 = &100;
    *ptr4 = 400;

    */
}
// maps
// values are mapped to key
// a map has a key and value
// O(1) -> maps are very efficient in terms of performance
// Hashing? -->"Hello World" 648a6a6ffffdaa0badb23b8baf90b6168dd16b3a
// echo "Hello World" | shasum
// When a map is create , it creates some buckets with the capacity
// each bucket is capable of holding an array or a linkedlist
// a key in a map cannot be duplicated
