use std::io;

fn main() {
    println!("Welcome to this interview by Ernst & Young Global Limited!");
    let mut applicants : Vec<String> = Vec::new();
    let mut experience : Vec<i32> = Vec::new();
    let mut count:i32 = 0;
    let mut choice:String = "y".to_string();

    while choice.to_lowercase() == "y" {
        count +=1 ;
        let mut input1 = String::new();
        println!("\nEnter your name applicant {} ", count);
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let new_applicant = input1.trim().to_string();
        applicants.push(new_applicant);

        let mut input2 = String::new();
        println!("How many years of work experience do you have? (Enter in numbers...) ");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let years_experience:i32 = input2.trim().parse().expect("Invalid input");
        experience.push(years_experience);

        let mut input3 = String::new();
        println!("Is there another applicant after you? (y/n)");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        choice = input3.trim().to_string();


    }
    if !experience.is_empty() {
        let mut max_value = experience[0];
        let mut max_index = 0;

        for (index, &value) in experience.iter().enumerate() {
            if value > max_value
             {
            max_value = value;
            max_index = index;
             }
        }
    
    println!("The highest years of experience is {} years.", max_value);
    println!("The applicant with the highest years of experience is {}.", applicants[max_index]);
    } else {
        println!("No applicants were entered.");
    }
}
