fn main() {

    let arr1 = [10,11,12,13,14,15,16,17,18,19];

    let slice1 = &arr1[..]; // all the elements of an array are referenced to the slice

    let slice2 = &arr1[3..8]; // all the elements between 3rd index - to 8th index but not 8th index

    let slice3 = &arr1[..4]; // all elements from 0 to 4 but not 4;

    let slice4 = &arr1[4..]; // all elements from 4 onwards

    let slice5 =  &arr1[3..=8]; // all elements from 3 to 8, including 8

    println!("slice1->{:?}",slice1);
    println!("slice2->{:?}",slice2);
    println!("slice5->{:?}",slice5);
    println!("slice3->{:?}",slice3);
    println!("slice4->{:?}",slice4);
    let sum1 = sum_of2(slice1);
    println!("slice1 sum:{}",sum1);
    let sum1 = sum_of2(slice2);
    println!("slice2 sum:{}",sum1);
    let sum1 = sum_of2(slice3);
    println!("slice3 sum:{}",sum1);

    let sum1 = sum_of2(slice4);
    println!("slice4 sum:{}",sum1);
    let sum1 = sum_of2(slice5);
    println!("slice5 sum:{}",sum1);

}

fn sum_of2(arr:&[i32])->i32{
    let mut sum =0;
    for i in arr.iter(){
        sum+= i;
    }
    return sum;
}
