fn main() {

    // If Expressions 

    let number = 4;
    if number > 5 {

        println!("Number is equal to {number}")
    }else{
        println!("Number is not equal greater 5")

    }

    // If expressions can be used to assign values
    let myage = 22;
    let agestate = if myage > 30 {
         "You are now old enough "
    } else{
         "You are still young , work on yourself"
    };

   
    println!("{}",agestate);
}
