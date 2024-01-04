fn main() {
    let primenumber = is_prime(2);
    println!("{}",primenumber);

    let arrayofnumbers:[u8;10] = [1,2,3,4,5,6,7,8,9,10];

    for number in arrayofnumbers {
        println!("{} is prime {}" , number , is_prime(number))
    }    
}


fn is_prime(number:u8)->bool{
    if number%2 == 0 {
    return false;
    }
    return true;
}

