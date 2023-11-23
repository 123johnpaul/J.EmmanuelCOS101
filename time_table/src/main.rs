use std::io;

fn main() {

    let mut input1 = String::new();
    let lecture_m1 = "mth101";
    let lecture_m2 = "phy107";
    let lecture_t1 = "sta111";
    let lecture_t2 = "phy102";
    let lecture_w1 = "csc192";
    let lecture_w2 = "sta111";
    let lecture_w3 = "sen114";
    let lecture_tt1 = "gst111";
    let lecture_tt2 = "cos101";
    let time_m1 = "9:00-11:00";
    let time_m2 = "14:00-17:00";
    let time_t1 = "10.00-12.00";
    let time_t2 = "14:00-16:00";
    let time_w1 = "9:00-11:00";
    let time_w2 = "12:00-1:00";
    let time_w3 = "15:00-17:00";
    let time_tt1 = "11:00-13:00";
    let time_tt2 = "14:00-17:00";
    let fri_read = "12:-16:00";
    let sat_read = "13:00-16:00";
    let sun_read = "11:00-15:00";
    let sun_mass = "9:00-10:30";
    let read_again = "18:00-20:00";
    let read_again_sun = "17:00-17;30";
    let breaktime = "30mins-1hr";

    println!("JP's time table");
    println!("which days of the week are required??");
    io::stdin().read_line(&mut input1).expect("not a character");
    let day = input1.trim();
    
        if day == "everyday of the week"{
            println!("Monday\t  \t {}\t {}\t {}\t {}\t ",lecture_m1,time_m1,lecture_m2,time_m2 );
            println!("Tuesday\t  \t {}\t {}\t {}\t {}\t ",lecture_t1,time_t1,lecture_t2,time_t2 );
            println!("Wednesday\t {}\t {}\t {}\t {}\t {}\t {}\t ",lecture_w1,time_w1,lecture_w2,time_w2,lecture_w3,time_w3 );
            println!("thursday\t {}\t {}\t {}\t {}\t ",lecture_tt1,time_tt1,lecture_tt2,time_tt2 );
        }else {
            println!("not existent");
        }
        println!("free time on monday 11:00 - 14:00. Read mth101 during lecture break, read phy 107 from 6:00-8:00");
        println!("free time on tuesday 12:00 - 14:00. Read sta111 during lecture break, read phy 101 from 6:00-8:00");
        println!("free time on Wednesday 11:00 - 12:00, 13:00-15:00. Read csc192 during lecture break for 30 mins, 
                 read sen 114 from 6:00-8:00");
        println!("no free time student fig session in free time, read cos 101 from 6:00-8:00");
        println!("friday\t washing {}\t reading {}\t break {}\t read again {}\t", time_m1,fri_read,breaktime,read_again);
        println!("saturday\t cooking {}\t reading {}\t break {}\t read again {}\t", time_m1,sat_read,breaktime,read_again);
        println!("sunday\t mass {}\t reading {}\t break {}\t read again {}\t", sun_mass,sun_read,breaktime,read_again_sun);
    
}
