// an input system that takes as input 
//the experience and age of employees
//in order to decide incentive for employees

use std::io;

fn main()
{
    let mut age = String::new();
    let mut input1 = String::new();
    println!("are you experienced?? (yes/no)");
    io::stdin().read_line(&mut input1).expect("not existent");
    let experienced = input1.trim();
    
    println!("enter your Age:");
    io::stdin().read_line(&mut age).expect("not existent");
    let Age:i32 = age.trim().parse().expect("not existent");

    if experienced == "yes"
    {
        if Age >= 40
        {
            println!("your incentive is N1_560_000");
        }
        else if Age >=30
        {
            println!("your incentive is N1_480_000");
        }
        else if Age <28
        {
            println!("your incentive is N1_300_000");
        }
    }
    if experienced == "no"
    {
        println!("your incentive is N100_000");
    }


}