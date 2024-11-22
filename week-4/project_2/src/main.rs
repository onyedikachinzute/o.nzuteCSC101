use std::io;

fn main() {

    println!("Please type in your experience level (e for experienced/i for inexperienced): ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read experience");
    let x:String = input1.trim().parse().expect("Failed to input");

    if x == "e" {
        println!("How old are you? ");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read age");
        let a:i32 = input2.trim().parse().expect("Failed to input age");

        if a >= 40 {
            println!("The incentinve of this employee should be 1,560,000 naira.");

        } else if a >= 29{
            println!("The incentive of this employee should be 1,480,000 naira.");

        } else {
            println!("The incentive of this employee should be 1,300,000 naira per month.");
            
        }
    } else if x == "i" {
        println!("The incentive for this inexperienced employee should be 100,000 naira per month.");

    }else {
        println!("You're probably a baby :( ");
    }


}