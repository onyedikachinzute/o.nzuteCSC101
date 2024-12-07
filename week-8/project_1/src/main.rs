use std::io;



fn main() {

let teacher = vec!["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"];
let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
let office_administrator = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];

let mut input1 = String::new();
println!("Which of the following are you? Type the corresponding number (1-4)");
println!("1. Teacher \n2. Lawyer \n3. Academic \n4. Office Administrator ");
io::stdin().read_line(&mut input1).expect("Failed to read input");
let office:i32 = input1.trim().parse().expect("Failed to read input");

if office == 1 {
    println!("Which of the following are you? Select 1-6");
    println!("1. {} \n2. {} \n3. {} \n4. {} \n5. {} \n6. {} \n", teacher[0], teacher[1], teacher[2], teacher[3], teacher[4], teacher[5]);
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let t:i32 = input2.trim().parse().expect("Failed to read input");

    if t == 1 {
        println!("You hold level APS 1-2");
    }else if t == 2 {
        println!("You hold level APS 3-5");
    }else if t == 3 {
        println!("You hold level APS 5-8");
    }else if t == 4 {
        println!("You hold level APS 8-10");
    }else if t == 5 {
        println!("You hold level APS 10-13");
    }else if t == 6 {
        println!("You hold level SES");
    }else {
        println!("You didn't input a number in the range");
    }

} else if office == 2 {
    println!("Which of the following are you? Select 1-6");
    println!("1. {} \n2. {} \n3. {} \n4. {} \n5. {} \n6. {} \n", lawyer[0], lawyer[1], lawyer[2], lawyer[3], lawyer[4], lawyer[5]);
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let t:i32 = input2.trim().parse().expect("Failed to read input");

    if t == 1 {
        println!("You hold level APS 1-2");
    }else if t == 2 {
        println!("You hold level APS 3-5");
    }else if t == 3 {
        println!("You hold level APS 5-8");
    }else if t == 4 {
        println!("You hold level APS 8-10");
    }else if t == 5 {
        println!("You hold level APS 10-13");
    }else if t == 6 {
        println!("You hold level SES");
    }else {
        println!("You didn't input a number in the range");
    }


} else if office == 3 {

    println!("Which of the following are you? Select 1-6");
    println!("1. {} \n2. {} \n3. {} \n4. {} \n5. {} \n6. {} \n", academic[0], academic[1], academic[2], academic[3], academic[4], academic[5]);
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let t:i32 = input2.trim().parse().expect("Failed to read input");

    if t == 1 {
        println!("You hold level APS 1-2");
    }else if t == 2 {
        println!("You hold level APS 3-5");
    }else if t == 3 {
        println!("You hold level APS 5-8");
    }else if t == 4 {
        println!("You hold level APS 8-10");
    }else if t == 5 {
        println!("You hold level APS 10-13");
    }else if t == 6 {
        println!("You hold level SES");
    }else {
        println!("You didn't input a number in the range");
    }
} else if office == 4 {

    println!("Which of the following are you? Select 1-6");
    println!("1. {} \n2. {} \n3. {} \n4. {} \n5. {} \n6. {} \n", office_administrator[0], office_administrator[1], office_administrator[2], office_administrator[3], office_administrator[4], office_administrator[5]);
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let t:i32 = input2.trim().parse().expect("Failed to read input");

    if t == 1 {
        println!("You hold level APS 1-2");
    }else if t == 2 {
        println!("You hold level APS 3-5");
    }else if t == 3 {
        println!("You hold level APS 5-8");
    }else if t == 4 {
        println!("You hold level APS 8-10");
    }else if t == 5 {
        println!("You hold level APS 10-13");
    }else if t == 6 {
        println!("You hold level SES");
    }else {
        println!("You didn't input a number in the range");
    }
} else {
    println!("That number isn't in the range");
}
}
