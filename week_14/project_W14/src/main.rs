use std::io;
use std::io::Read;


fn main() {
    println!("Good day to you");
    let mut input = String::new();

    println!("what is your role/position in company??\n Administrator\nProject Manager\nEmployee\nCustomer\nVendor\nensure first letter of position is in capital letter\n\nInput position:");
    io::stdin().read_line(&mut input).expect("not valid");
    let role = input.trim();

    if role == "Administrator"{
        let mut file = std::fs::File::open("globacom_database.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }else if role == "Project Manager"{
        let mut file = std::fs::File::open("project_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }else if role == "Employee"{
        let mut file = std::fs::File::open("staff_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }else if role == "Customer"{
        let mut file = std::fs::File::open("customer_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }else if role == "Vendor"{
        let mut file = std::fs::File::open("dataplan_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }
}
