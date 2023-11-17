use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    for _ in 0..500{
        println!("Nigerian Researchers Guide(NRG) incentive check");
        println!("enter your name");
        io::stdin().read_line(&mut input1).expect("not a character");
        let Name = input1.trim();

        println!("Number of papers published;");
        io::stdin().read_line(&mut input2).expect("not a character");
        let papers:i64 = input2.trim().parse().expect("not a character");

       if papers < 3{
        println!("Name: {}", Name);
        println!("incentive earned: N100,000");
       }else if papers >= 3 && papers <= 5{
        println!("Name : {}", Name);
        println!("incentive obtained: N500,000");
       }else if papers > 5 && papers < 10{
        println!("Name : {}", Name);
        println!("incentive obtained: N800,000");
       }else if papers >= 10{
        println!("Name : {}", Name);
        println!("incentive obtained: N1,000,000");
       }
    }
}