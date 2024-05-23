fn main() {
    
    let arr1 = [10,11,12,13,14,15]; // array signature contains the size . arr1 is owned type
    // array header
    // ptr length capacity

    let slice1 = &arr1[..]; // slice signature does not contain the size. slice1 is a reference type

    println!("{}",arr1[0]);

    for i in arr1.iter(){
        println!("{}",i);
    }

    for i in 0.. arr1.len(){
        println!("{}",arr1[i]);
    }

    let arr1 = [10,11,12,13,14,15]; // array signature contains the size . arr1 is owned type

    let arr2 = [10,11,12,13,14]; // array signature contains the size . arr1 is owned type

    let sum1 = sum_of1(arr1);
    let sum2 = sum_of2(&arr2);
    let sum3 = sum_of2(&arr1);

    println!("{} {} {}",sum1,sum2,sum3);

}

fn sum_of1(arr:[i32;6])->i32{

    let mut sum =0;
    
    for i in arr.iter(){
        sum+= i;
    }

    return sum;
}

fn sum_of2(arr:&[i32])->i32{

    let mut sum =0;
    
    for i in arr.iter(){
        sum+= i;
    }

    return sum;
}

// fn sum_of3(arr:&[i32])->i32{
//     if arr.len()!=6{
//         return -1;
//     }
//     let mut sum =0;
    
//     for i in arr.iter(){
//         sum+= i;
//     }

//     return sum;
// }




// array is an collection of valus of same type
// arrays are fixed size
// arrays are allocated on stack
// you can create a slice to an array. 
// the index of an array 

// slice
// slice is dynamically sized collection of elemernts of an array or vector
// slice do not own their data; becase they are references
// &str is a string slice


// two types in rust 
// owned types and reference types/
// let x = 100; // x is a owned type
// let y = &x; // y is a reference type
