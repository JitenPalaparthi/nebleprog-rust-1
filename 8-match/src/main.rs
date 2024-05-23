fn main() {
    let day = 4;

    match day {
        1 => println!("Sunday"),
        2 => println!("Monday"),
        3 => println!("Tuesday"),
        4 => {
            println!("Wednesday")
        }
        5 => println!("Thursday"),
        6 => println!("Friday"),
        7 => println!("Saturday"),
        _ => {
            println!("no day");
        }
    }

    let num = 2;

    match num {
        1..=5 => {
            println!("The value is between 1-5");
        }
        _ => {}
    }

    let num = 2; // shadow num

    match num {
        1 | 2 | 3 => {
            println!("The value is 1 or 2 or 3");
        }
        _ => {
            // default case equallent
            println!("not 1 or 2 or 3");
        }
    }

    let tuple1 = (0, -2);

    match tuple1 {
        (0, y) => println!("The first value is zero and y is {}", y),
        (x, 0) => println!("The first value is x {}  and second is zero ", x),
        _ => {}
    }

    let number = 201;
    let divider = 2;
    match number {
        x if x % divider == 0 => println!("even"),
        y if y % divider != 0 => println!("odd"),
        _ => {}
    }
}

// 1..5 1,2,3,4
// 1..=5 1,2,3,4,5
