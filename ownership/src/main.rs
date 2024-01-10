fn main() {
    let s1 = String::from("Hello World");
    let _s2 = s1;
    //println!("{}",s2);
    let name = String::from("Mophat");
    let myage:u8 = 21;
    print_name(String::from(name));
    print_age(myage);
    println!("I am {} years old",myage); // My age is still in scope even after passing it into a function .
    //println!("{}",name) name is now out of scope has it has been moved .
}


fn print_name (name:String){
    println!("Hello {}",name);
}

fn print_age (myage:u8){
    println!("I am {} years old",myage);
}
