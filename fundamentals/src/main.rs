fn main() {
// Single Lined comments and Multiline Comments .
// Lets discuss on Println! macro and println! is a macro that prints text to the console.

/* As mentioned in the “Storing Values with Variables” section, by default, variables are immutable. This is one of many nudges Rust gives you to write your code in a way that takes advantage of the safety and easy concurrency that Rust offers. However, you still have the option to make your variables mutable. Let’s explore how and why Rust encourages you to favor immutability and why sometimes you might want to opt out.
oiwerefuygjhh9328beruifhjbn-90o3irj
When a variable is immutable, p[1weokjf
wp32eorif
wklejrnfbv
qwdmnf] */


//println!("Hello, world!");


// print!("Hello, world!");
// print!("Hello, world!");


// println!("Hello, world!");
// println!("Hello, world!");


// Variables in Rust .
// let mut age = 22;

// age = 50;





// let _name = "Mophat";


// println!( "I am {_name} and I am {age} years old");
// println!( "I am {_name} and I am {} years old", age);


// SCALAR  DATATYPES 
// Intergers ,floating point numbers , booleans , character .
// INTEGERS 
// i8 , i16 , i32 , i64,  + or -
//u8 , u16 , u32 , u64  , unsigned meaning we can only have + 
// 0 and 1 are called binary numbers .
// 2^7 => 2^7 - 1 = range -128 - 127
// let size:i64  = 49;
// let number: u32 = 50; 
// let age:i8 = 22;




// Mutable variables and  Immutable Variables 

// This is is an Immutable variable .
let _age1: i32 = 29;

// This is a Mutable variable .

let mut age2:i8 = 30;

age2 = 45;

// println!("{age2}");

// Floating Point Numbers 
// f32 and f64 .

let mut temprature:f64 = 30.123;
let pi: f64 = 3.142;
temprature = 45.08;
// println!("{}",temprature);

// Booleans 


let isCooking:bool = true;
let isCryrying: bool = false;

// Characters in Rust


let mut vowel:char = 'a';

// COMPOUND DATATYPES.


let numbers: [i32;11] = [1,2,3,4,5,6,7,8,9,11,12];

let number1: i32 = numbers[0];
let number2:i32 = numbers[1];


let mut months:[&str;2] = ["January","February"];
months[1] = "March";



let mut secondMonth:&str = months[0];

// println!("{secondMonth}");


// Tupples .

let mut tup1:(i32,bool,char,&str) = (34,true,'o',"Mophat");


tup1.1 = true;

let secondValue:bool = tup1.1;

println!("{secondValue}");
                         



}
