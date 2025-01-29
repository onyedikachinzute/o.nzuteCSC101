use std::io;
use std::io::Read;

fn main() {

    let mut input1 = String::new();
    println!("Welcome user!");
    println!("Which of the following are you?");
    println!("1. Administrator \n2. Project Manager \n3. Employee \n4. Customer \n5. Vendor \n");
    io::stdin().read_line(&mut input1).expect("Failed to read file");
    let choice:i32 = input1.trim().parse().expect("Failed to read input.");

    if choice ==1 {
        let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }else if choice == 2 {
        let mut file = std::fs::File::open("project_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }else if choice == 3 {
        let mut file = std::fs::File::open("staff_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }else if choice == 4 {
        let mut file = std::fs::File::open("customer_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }else if choice == 5 {
        let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }else {
        println!("Selection not in the range above!");
    }

}
