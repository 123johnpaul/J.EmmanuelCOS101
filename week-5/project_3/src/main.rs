//food menu for resturant

use std::io;

fn main()
{
    let mut bill:f64 = 0.0;

    println!("welcome to our resturant");
    println!("what is your order??");
    println!("food avalible include");
    println!("pounded yam & edinkaiko soup(input p to order) @ N3_500");
    println!("fried rice & chicken(input f to order) @ N3_000");
    println!("amala & ewedu soup(input a to order) @ N2_500");
    println!("eba & egusi soup(input e to order) @ N2_000");
    println!("white rice & stew(input w to order) @ N2_500");
    println!("input c to cancel order or to get bills for the order placed");
    println!("Note:
        to order a certain item twice input it twice in the system 
        so to say you input the letter for that meal once again after clicking enter");

    loop{
        let mut food = String::new();
    io::stdin().read_line(&mut food).expect("not availible");
    let food = food.trim();

    if food == "p"{
         bill+=3500.0; 
    } else if food == "f"{
         bill+=3000.0;
    }else if food == "a"{
         bill+=2500.0;
    }else if food == "e"{
         bill+=2000.0;
    }else if food == "w"{
         bill+=2500.0;
    }
    else if food == "c"{
        break;
    }else {
        println!("so sorry to say this but your order isn't on sale today");
        continue;
    }
    }

    if bill > 10000.0{
        let discount_bill:f64 = bill - (5.0 / 100.0) * bill;
        println!("Note:
            Because you made a purchase of above N10,000
            You have earned a discount of 5% on your bill");
        println!("your bill is N{}", discount_bill);
    }else {
        println!("Your bill is N{}", bill);
    }

    println!("Thanks for your patronage...");
    println!("your can book us for occasions via whatsapp (09132348494)");
    
}