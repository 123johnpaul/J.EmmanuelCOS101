// Rust program to determine age pass

use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("not a valid string");

    println!("Enter your age:");
    io::stdin().read_line(&mut input2).expect("invalid");
    let age:i32 = input2.trim().parse().expect("invalid");

    if age >= 18 {
         println!("Welcome to the party mf {}!", input1 );
    }    else {
        println!("shey na me you wan whine go find your mama your mates no dey here {}", input1);
    }

}
