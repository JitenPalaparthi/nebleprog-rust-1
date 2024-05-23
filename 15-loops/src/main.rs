fn main() {
    println!("Loop using loop keyword");
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Normal:{}", counter);
        if counter % 2 == 0 {
            println!("Even:{}", counter)
        } else {
            println!("Odd:{}", counter);
        }
        if counter == 10 {
            break;
        }
    }

    println!("Loop using while keyword");
    let mut number = 1;
    while number <= 10 {
        println!("Normal:{number}");
        if number % 2 == 0 {
            println!("Even:{}", number);
        } else {
            println!("Odd:{}", number);
        }
        number+=1;
    }

    println!("Loop using for keyword");
    for number in 1..=10{ // 1..10 from 1 to 10 but not 10. 1..=10 from 1 to 10 including 10
        println!("Normal:{number}");
        if number % 2 == 0 {
            println!("Even:{}", number);
        } else {
            println!("Odd:{}", number);
        }

    }
    println!("nested loops");

    for i in 1..3{ // for each iteration of i(outer loop)
        for j in 1..3{ // the full loop  j (inner loop)iterates.
            println!("i:{i},j:{j}");
        }
    }

    println!("nested loops break");

    'o:  for i in 1..3{ // for each iteration of i(outer loop)
        for j in 1..3{ // the full loop  j (inner loop)iterates.
            if i==1{
                break 'o; // if only break break is given it only breaks the innerloop.. to break both the loops you can use break with a lable
            }
            println!("i:{i},j:{j}");
        }
    }
    println!("iterate through charaters");
    let name = "NobleProg";

    for c in name.chars(){
        print!("{}:{}  ",c, c as i8)
    }
    println!();

    for c in name.bytes(){
        print!("{}:{}  ",c as char, c)
    }
    println!();

    let name="نبيلة Noble";

    for c in name.chars(){
        print!("{}:{}  ",c, c as i8)
    }
    println!();

    for c in name.bytes(){
        print!("{}:{}  ",c as char, c)
    }
    println!();

}
