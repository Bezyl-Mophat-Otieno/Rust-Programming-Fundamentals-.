fn main() {
    let far = 20;
    println!("Converting {} degrees celcius = {} Farenheits" , far , degreetofarenheit(far))
}


fn degreetofarenheit (degree:i8)->i8{
    let farenheit:i8 = degree+32;
    return farenheit;
}