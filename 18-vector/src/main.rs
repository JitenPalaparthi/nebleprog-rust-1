fn main() {
    let mut vec1 = vec![10, 11, 12, 13, 14, 15]; // creating a vector using vec! macro

    let vec2: Vec<i32> = Vec::new(); // creating a vector using Vec type

    vec1[0] = 100; // mutate the existing vec element with a new value. The vec should be mutable

    println!("length:{}", vec1.len());
    println!("Capacity:{}", vec1.capacity());

    vec1.push(16);
    vec1.push(17);
    vec1.push(18);
    vec1.push(19);
    vec1.push(20);
    vec1.push(21);
    vec1.push(22);
    println!("length:{}", vec1.len());
    println!("Capacity:{}", vec1.capacity());
    vec1.pop();
    vec1.pop();
    vec1.pop();
    vec1.pop();
    vec1.pop();
    vec1.pop();
    vec1.pop();
    println!("length:{}", vec1.len());
    println!("Capacity:{}", vec1.capacity());

    let mut sum = 0;
    for n in vec1.iter() {
        //sum += *n; // rust is making our life easy even * is omited , rust can understand
        sum+=n;
    }

    println!("Sum:{}",sum);

    let mut sum = 0;
    for n in vec1 {
        sum += n;
    }
    println!("Sum:{}",sum);

    println!("Multi-dimentional vector");

    let vec2d = vec![vec![1,2] ,vec![3,4]]; 

    for v1 in vec2d{
        for v2 in v1{
            print!(" {} ",v2);
        }
    }

    println!("Multi-dimentional(3d) array");
    let arr2d = [[[1,2],[3,4]],[[5,6],[7,8],],];

    for v1 in arr2d{
        for v2 in v1{
           for v3 in v2{
            print!(" {} ",v3);
           }
        }
    }
    let mut vec1 = vec![10, 11, 12, 13, 14, 15]; 

    let slice1 = &vec1[..];
    let slice2 = &vec1[2..=5];
println!();
   let  sum1 = sum_of2(slice1);
   println!("sum of slice :{}",sum1);
   let  sum1 = sum_of2(&vec1);
   println!("sum of vec :{}",sum1);
   let arr1 = [1,2,3,4];
   let  sum1 = sum_of2(&arr1);
   println!("sum of arr:{}",sum1);
  

}

// vector is a dynamically sized array
// vector is stored in heap
// can create a slice from a vector
// vector can grow or shrink
// there is a length and also a capacity for vector


fn sum_of2(arr:&[i32])->i32{
    let mut sum =0;
    for i in arr.iter(){
        sum+= i;
    }
    return sum;
}
