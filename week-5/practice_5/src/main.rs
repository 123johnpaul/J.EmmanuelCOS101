// Rust program to read the height of a person
// and then print if the person is tall, dwarf,
// or average height person

use std::io;

fn main() 
{
    let mut input1 = String::new();

    println!("\nEnter your height (in centimeters):");
    io::stdin().read_line(&mut input1).expect("invalid");
    let height:f32 = input1.trim().parse().expect("invalid");

    if height >= 150.0 && height <=170.0
    {
        println!("your are average in height");
    }
    else if height > 170.0 && height <= 195.0
    {
        println!("tall ass MF!!!");
    }
    else if height < 150.0 && height >100.0
    {
        println!("you are ground level MF!!!");
    }
    else 
        {
            println!("GET YOUR ABNORMAL ASS OUTTA HERE");
        }
    
}
