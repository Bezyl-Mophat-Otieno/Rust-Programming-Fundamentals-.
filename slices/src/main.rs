fn main() {
    // Creating a function that will return a slice of a string passed to it when it encounters a space

    let my_name = String::from("Bezyl Mophat");

    let first_name = word_slice(&my_name);
    println!("I am {}", first_name);
}

fn word_slice(s: &String) -> &str {
    // breakdown the string into bytes
    let bytes = s.as_bytes();

    // iterate through the bytes returning the slice of the string when it encounters a space
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
