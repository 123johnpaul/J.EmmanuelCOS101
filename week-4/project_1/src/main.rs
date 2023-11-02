use std::io;

fn main() 
{
    let mut number_of_miles = String::new();
    let mut time_taken_in_hours = String::new();
    
    println!("Enter number of miles");
    io::stdin().read_line(&mut number_of_miles).expect("not valid");
    let m:f32 = number_of_miles.trim().parse().expect("invalid");

    println!("Enter time taken in hours to cover distance");
    io::stdin().read_line(&mut time_taken_in_hours).expect("not valid");
    let t:f32 = time_taken_in_hours.trim().parse().expect("invalid");

    let speed:f32 = (m * 1.60934) / t;
    println!("speed of vehicle is: {}",speed );


}
