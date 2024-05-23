fn main() {

    let age = 28;
    let gender: char = 'F';
        // true && (true)
        // true
    if age >=18 && (gender=='F' || gender=='f'){
        println!("In India, she is elegible for marriage");
    } else if age >=21 && (gender=='M' || gender=='m'){
        println!("In India, He is eligible for marriage");
    }else{
        println!("Unidentified gender or age");
    }

    // another explanation about expression in if 

    // expression returns a value but a statement does not return a value but execites the statement

    // if true{

    // } // false 
    // else{

    // }

    let y = { // this is an expression, this expression is evaluated and assigned to the variable y
        let x = 3;
       ( x * x ) as f32
    };

    println!("y={y}");

    let number = 208;
    let result = if number %2 ==0{
        "even"
        //true
        //2
    }else{
        "odd"
      // false
      //1
    };

    println!("{}",result);

}
