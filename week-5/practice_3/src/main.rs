// Rust program to calculate the area of a triangle for a given base and height

use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter base: ");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let base:f32 = input1.trim().parse().expect("invalid");

    println!("Enter height:");
    io::stdin().read_line(&mut input2).expect("invalid");
    let height:f32 = input1.trim().parse().expect("invalid");

    if base > 0.0 {
         let mut area:f32 = (base * height) / 2.0;
         println!("Area of triangle: {}", area );
    }

}