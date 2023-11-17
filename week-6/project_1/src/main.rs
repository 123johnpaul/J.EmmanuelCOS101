//code to check eligibility of voters

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();

    for _ in 0..150{
    println!("enter your name");
    io::stdin().read_line(&mut input1).expect("not a character");
    let Name = input1.trim();

    println!("Are you a course rep(yes/no)");
    io::stdin().read_line(&mut input2).expect("not a character");
    let course_rep = input2.trim();

    println!("what level are you in??");
    io::stdin().read_line(&mut input3).expect("not a character");
    let level:i64 = input3.trim().parse().expect("not a character");

    println!("what department are you in");
    io::stdin().read_line(&mut input4).expect("not a character");
    let department = input4.trim();

    println!("what is your C.G.P.A??");
    io::stdin().read_line(&mut input5).expect("not a character");
    let CGPA:f64 = input5.trim().parse().expect("not a character");

    println!("enter your student email;");
    io::stdin().read_line(&mut input6).expect("not a character");
    let student_email = input6.trim();

    println!("State of origin;");
    io::stdin().read_line(&mut input7).expect("not a character");
    let State_of_origin = input7.trim();

    if course_rep == "yes"{
        if level > 100{
            if CGPA > 4.0{
                println!("you are eligible to vote");
            }else {
                println!("you are not eligible to vote");
            }
        }
    }

    }
   
}
