fn main(){
    let mut m1 = Message::Quit;
    let m2 = Message::Move{x:100,y:200};
    let m3 = Message::Send("Hello World".to_string());
    // print_message(&m1);
    // print_message(&m2);
    // print_message(&m3);

    // m1 = Message::Move{x:1000,y:2000};
    // print_message(&m1);
    // print_message(&m1);

    m1.print();
    m1.move_to(112, 113);
    m1.print();
}

// if you want to create methods
// use impl block
// this 

impl Message{
    fn print(&self){
        match self{
            Message::Quit=>{
                println!("{:?}",self);
            }
            Message::Move { x, y }=>{
                println!("X:{x} Y:{y}");
            }
            Message::Send(txt)=>{
                println!("{txt}")
            }
        }
    }
    fn move_to(&mut self,x:i32,y:i32){
        *self = self::Message::Move{x:x,y:y};
    }
}

fn print_message(m:&Message){
    match m{
        Message::Quit=>{
            println!("Message: {:?}",m);
        }
        Message::Move { x, y } =>{
            println!("X:{} Y:{}",x,y);
        }
        Message::Send(txt)=>{
            println!("{}",txt);
        }
    }
} 

#[derive(Debug)]
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Send(String),

    // This is not the place for methods
}