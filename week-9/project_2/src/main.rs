use std::io::Write;

fn main() {

    let mut name = Vec::new();
    let mut matricno = Vec::new();
    let mut dept = Vec::new();
    let mut lvl = Vec::new();
    let mut count = 0;
    let mut choice = String::from("y");

    println!("This is the PAU SMIS Print Service!");
    println!("This will print out the details of all the students you enter into this program!\n");

    while choice == "y" {

        count += 1;
        let mut input1 = String::new();
        println!("\n\nWhat is your name, student {}?", count);
        std::io::stdin().read_line(&mut input1).expect("Failed to read input");
        let new_name = input1.trim().to_string();
        name.push(new_name);

        let mut input2 = String::new();
        println!("Please input your Matriculation Number.");
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_matricno = input2.trim().to_string();
        matricno.push(new_matricno);

        let mut input3 = String::new();
        println!("Please input your Department.");
        std::io::stdin().read_line(&mut input3).expect("Failed to read input");
        let new_dept = input3.trim().to_string();
        dept.push(new_dept);

        let mut input4 = String::new();
        println!("What Level are you in? Please input in numbers.");
        std::io::stdin().read_line(&mut input4).expect("Failed to read input");
        let new_lvl = input4.trim().to_string();
        lvl.push(new_lvl);

        let mut input5 = String::new();
        println!("Are there more students inputting their details? y/n");
        std::io::stdin().read_line(&mut input5).expect("Failed to read input");
        choice = input5.trim().to_string();
    }

    println!("The PAU SMIS database is being created...");

    let mut file = std::fs::File::create("pau_smis.txt").expect("create failed");
    file.write_all("PAU SMIS\n\n".as_bytes()).expect("Write failed");
    file.write_all(format!("{:<15} {:<20} {:<25} {:<15}\n", "Student Name", "Matric. Number", "Department", "Level").as_bytes()).expect("write failed");
    let max_rows = name.len();

    for i in 0..max_rows {
    
        let name_entry = name.get(i).map(String::as_str).unwrap_or("Unknown"); 
        let matric_entry = matricno.get(i).map(String::as_str).unwrap_or("Unknown");
        let dept_entry = dept.get(i).map(String::as_str).unwrap_or("Unknown");
        let lvl_entry = lvl.get(i).map(String::as_str).unwrap_or("Unknown");

        file.write_all(format!("{:<15} {:<20} {:<25} {:<15}\n", name_entry, matric_entry, dept_entry, lvl_entry).as_bytes()).expect("write failed");
    }

    println!("The PAU SMIS has been created successfully!\n\n\n");
    println!("If you want to use this program, please contact Nzute Onyedikachi at the following number:");
    println!("+2349031713889");
    println!("Thank You!");

}
