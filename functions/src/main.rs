fn main() {
    // Expressions and statements 

    // Error assigning statements 

    // let x = (let y=8);

    // An expression can be part of a statement 

    let _y:i32 = 9;


// Calling a function is an expression 
    say_hello("Mophat");

// Expressions separated in scope block 
    let y:i32 = {
        let x:i32=3;
        x+1
    };

    println!("{}",y);

    // Calculating my age today

    let myage:i32 = age_calculator(2002);
    println!("I am Mophat , and I am {myage} years old")
}

fn say_hello(name:&str){
    println!("Hello {name}")
}

// Functions with Returning Values
 
 fn age_calculator (year_of_birth:i32)-> i32 {

    2023-year_of_birth

 }

