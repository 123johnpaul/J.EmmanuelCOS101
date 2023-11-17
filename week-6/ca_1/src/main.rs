//Rust code for medical and personal information

use std::io;

fn main() {
    let mut hospital_bill:f64 = 0.0;
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();
    let mut input9 = String::new();
    let mut input10 = String::new();

   for _ in 0..100{
     println!("enter your name");
    io::stdin().read_line(&mut input1).expect("not a character");
    let Name = input1.trim();

    println!("enter your date of birth (dd/mm/yy)");
    io::stdin().read_line(&mut input2).expect("not a character");
    let dob:i32 = input2.trim().parse().expect("not a character");

    println!("Your phone number");
    io::stdin().read_line(&mut input3).expect("not a character");
    let phone_number:f64 = input3.trim().parse().expect("not a character");

    println!("number of siblings");
    io::stdin().read_line(&mut input4).expect("not a character");
    let number_of_siblings:f64 = input4.trim().parse().expect("not a character");

    println!("number of children");
    io::stdin().read_line(&mut input5).expect("not a character");
    let number_of_children:f64 = input5.trim().parse().expect("not a character");

    println!("enter your date of birth (dd/mm/yy)");
    io::stdin().read_line(&mut input6).expect("not a character");
    let dob:f64 = input6.trim().parse().expect("not a character");

    println!("medical diagnosis");
    println!("Alzheimer/Arrhythmia/Chronic kidney disease/Diabetes/Arthritis");
    io::stdin().read_line(&mut input7).expect("not a character");
    let Medical_diagnosis = input7.trim();

    println!("enter your village of residence");
    println!("Akpabom/Ngbauji/Atabrikang/Okorobilom/Emeremen");
    io::stdin().read_line(&mut input8).expect("not a character");
    let Village_of_residence = input8.trim();

    println!("enter your age");
    io::stdin().read_line(&mut input9).expect("not a character");
    let Age:i64 = input9.trim().parse().expect("not a character");

    println!("enter your email");
    io::stdin().read_line(&mut input10).expect("not a character");
    let email = input10.trim();


    if Age > 50{
        if Medical_diagnosis == "Alzheimer"{
            if number_of_children > 4.0 {
                if Village_of_residence == "Akpabom"{
                    let new_hospital_bill = hospital_bill - ((20.0/100.0) * hospital_bill);
                println!("you have earned a 20% discount because you meet the requirements");
                println!("paitent name; {}", Name);
                println!("date of birth; {}", dob);
                println!("email address; {}", email);
                println!("phone number; {}", phone_number);
                println!("number of siblings; {}", number_of_siblings);
                println!("number of children; {}", number_of_children);
                println!("Medical_diagnosis; {}", Medical_diagnosis);
                println!("Village_of_residence; {}", Village_of_residence);
                println!("hospital_bill; {}", hospital_bill);
                println!("new_hospital_bill; {}", new_hospital_bill); 
                }
            }
        }
    }else {
        println!("hospital_bill; {}", hospital_bill);
     }

    if Age == 30{
        if Medical_diagnosis == "Arrythmia"{
            if number_of_siblings > 4.0 {
                if Village_of_residence == "Ngbauji"{
                    let new_hospital_bill = hospital_bill - ((5.0/100.0) * hospital_bill);
                println!("you have earned a 5% discount because you meet the requirements");
                println!("paitent name; {}", Name);
                println!("date of birth; {}", dob);
                println!("email address; {}", email);
                println!("phone number; {}", phone_number);
                println!("number of siblings; {}", number_of_siblings);
                println!("number of children; {}", number_of_children);
                println!("Medical_diagnosis; {}", Medical_diagnosis);
                println!("Village_of_residence; {}", Village_of_residence);
                println!("hospital_bill; {}", hospital_bill);
                println!("new_hospital_bill; {}", new_hospital_bill); 
                }   
            }
        }
    }else {
        println!("hospital_bill; {}", hospital_bill);
    }

    if Age > 40{
        if number_of_children > 3.0 {
            if number_of_siblings > 3.0{
                if Village_of_residence == "Atabrikang"{
                    let new_hospital_bill = hospital_bill - ((15.0/100.0) * hospital_bill);
                println!("you have earned a 15% discount because you meet the requirements");
                println!("paitent name; {}", Name);
                println!("date of birth; {}", dob);
                println!("email address; {}", email);
                println!("phone number; {}", phone_number);
                println!("number of siblings; {}", number_of_siblings);
                println!("number of children; {}", number_of_children);
                println!("Medical_diagnosis; {}", Medical_diagnosis);
                println!("Village_of_residence; {}", Village_of_residence);
                println!("hospital_bill; {}", hospital_bill);
                println!("new_hospital_bill; {}", new_hospital_bill); 
                }
            }
        }
    }else {
        println!("hospital_bill; {}", hospital_bill);
    }

    if Age > 28 {
        if Medical_diagnosis == "Diabetes"{
            if number_of_siblings > 4.0 {
                if Village_of_residence == "Okorobilom"{
                    let new_hospital_bill = hospital_bill - ((10.0/100.0) * hospital_bill);
                println!("you have earned a 10% discount because you meet the requirements");
                println!("paitent name; {}", Name);
                println!("date of birth; {}", dob);
                println!("email address; {}", email);
                println!("phone number; {}", phone_number);
                println!("number of siblings; {}", number_of_siblings);
                println!("number of children; {}", number_of_children);
                println!("Medical_diagnosis; {}", Medical_diagnosis);
                println!("Village_of_residence; {}", Village_of_residence);
                println!("hospital_bill; {}", hospital_bill);
                println!("new_hospital_bill; {}", new_hospital_bill); 
                }
            }
        }
    }else {
        println!("hospital_bill; {}", hospital_bill);
    }

    if Age > 58{
        if number_of_children > 5.0 {
            if number_of_siblings > 5.0{
                if Village_of_residence == "Emeremen"{
                    let new_hospital_bill = hospital_bill - ((10.0/100.0) * hospital_bill);
                println!("you have earned a 10% discount because you meet the requirements");
                println!("paitent name; {}", Name);
                println!("date of birth; {}", dob);
                println!("email address; {}", email);
                println!("phone number; {}", phone_number);
                println!("number of siblings; {}", number_of_siblings);
                println!("number of children; {}", number_of_children);
                println!("Medical_diagnosis; {}", Medical_diagnosis);
                println!("Village_of_residence; {}", Village_of_residence);
                println!("hospital_bill; {}", hospital_bill);
                println!("new_hospital_bill; {}", new_hospital_bill);
                } 
            }
        }
    }else {
        println!("hospital_bill; {}", hospital_bill);
    }




    }

   }