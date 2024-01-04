
fn main() {

    // Loops , break and Continue

    // loop {

    //     println!("Hello world");

    // }

    // Break from the Loop .

    // let mut counter = 0;

    // let result:i32 = loop{

    //     counter += 1;

    //     if counter ==10{
    //         break counter*2;
    //     }
    // };

    // print!("{}",result);


    //  Labelling Loops 

    // let mut count = 0;

    // 'count_up: loop{
    //     println!("Count = {count}");

    //     let mut remaining: i32 = 10;  
    //     'count_down: loop{
    //         println!("Remaining = {remaining}");

    //         if remaining == 0 {
    //             break;
    //         }

    //         if count == 10 {
    //             break 'count_up;
    //         }
    //         remaining-=1;
    //     }
        
    //     count += 1;
    // }

    // Using a while loop

    // let mut count: i32 = 0;

    // let mut remaining :i32 = 10;


    // while count != 10 && remaining !=0 {
    //     println!("Count = {count}");
     
    //  println!("Remaining = {remaining}");

    //  count +=1;
    //  remaining -=1;

    // }

    // println!("Lifted Off");

    // For Each Loop 

    // let numbers: [i32;5] = [12,3,4,5,5];

    // for number in numbers {
    //     println!("{number}");
    // }

    // Using the range in RUST

    for number in (1..4).rev(){
        println!("{number}");
    }
}
