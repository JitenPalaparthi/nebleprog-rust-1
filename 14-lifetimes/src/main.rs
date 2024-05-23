fn main() {
    bigger_str("Hello, world!","Hello world ..");
}

// ownership
// borrowing
// references
// scope
// mutable and immutable borrow
// lifetimes

fn bigger_str<'a>(x:&'a str,y:&'a str)->&'a str{
    if x.len()>y.len(){
       return x;
    }
    return y;
}

fn both_str<'a>(x:&'a str,y:&'a str)->(&'a str,&'a str){
    // if x.len()>y.len(){
    //    return (x,y);
    // }
    return (x,y);
}

fn get_length1<'mylifetime>(s:&'mylifetime str)->i32{ // no need to give lifetime here becase rust can automatically understand the lifetime of a reference
    let k = 100;
    //et l:&i32 = &k
    //return l;
    return k;
}

fn get_length2(s1:&str,s2:&str)->i32{ // no need to give lifetime here becase rust can automatically understand the lifetime of a reference
    (s1.len()+s2.len())as i32
}