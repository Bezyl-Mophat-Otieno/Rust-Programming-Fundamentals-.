fn main() {
    // Creating a string
    let my_name = String::from("Bezyl Mophat");

    /* Rather than passing the entire ownership to the name_length funtion , we only pass a reference
    to the the parameter tha't maintaining the ownership of the my_name to my_name and only pass in the borrowed representation of the my_name string
    */

    let length = name_length(&my_name);
    println!("{}", length);
    /*  Since ownership was'nt transferred the my_name variable is still in scope here.... */
    println!("{}", my_name);

    let mut _currentage: u8 = 22;
    let mut ageinformation = String::from("I am 30 years old");

    age_change(&mut ageinformation);

    println!("{}", ageinformation);
}

fn name_length(name: &String) -> usize {
    return name.len();
} /* Here name goes out of scope but since it's a mere reference to the actual variable
  it's not dropped at this point.
  */

fn age_change(information: &mut String) {
    information.push_str(", this is modified");
}
