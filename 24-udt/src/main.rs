fn main() {

    let b1 = Box::new(100);
    let s1 = String::new();
    let s2 = String::from("Hello");
    let s3 = String::from_utf8(vec![34,45,65]).unwrap();
    let s4 = String::from_utf8(vec![34,45,65]).expect("not received a proper string");
    let s5: Result<String, std::string::FromUtf8Error> =  String::from_utf8(vec![34,45,65]);

    // match s5{
    //     Err(e)=>{
    //         println!("{:?}",e);
    //     }
    //     Ok(v)=>{
    //         print!("{v}");
    //     }
    // }


    let p1 = Person::new("Jiten".to_string(),39,"JitenP@Outlook.com".to_string(),true);
    let s1 = Student::new("Jiten",39,"JitenP@Outlook.com",true);

    // s1 is created , the memory is allocated for s1 and also there are some fields which are references
}


struct Person{
    name:String,
    age:u8,
    email: String,
    is_married:bool,
    //address:Address,
}


impl Person{
    fn new(n:String,a:u8,e:String,im:bool)->Self{ // &self and Self 
    //    let p = Person { name: n, age: a, email: e, is_married: im };
    //    return p;
    Person { name: n, age: a, email: e, is_married: im }
    }

    fn default()->Self{ // &self and Self 
        //    let p = Person { name: n, age: a, email: e, is_married: im };
        //    return p;
        Person { name: "".to_string(), age: 0, email: "".to_string(), is_married: false }
        }
}

// struct Address{
//     city:String,
//     zipcode:String,
// }



struct Student<'a>{
    name:&'a str, // what &str --> string slice a.k.a reference to a stored string
    age:u8,
    email: &'a str,
    is_married:bool,
    //address:Address,
}


impl<'a> Student<'a>{
    fn new(n:&'a str,a:u8,e:&'a str,im:bool)->Self{ // &self and Self 
    Student { name: n, age: a, email: e, is_married: im }
    }
}
