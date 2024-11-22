use std::io;

fn main() {
    
    println!("The basic format of a quadratic equation is ax^2 + bx + c = 0");
    println!("Input the value of a: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read a");
    let a:f32 = input1.trim().parse().expect("Failed to input");

    println!("Input the value of b: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read b");
    let b:f32 = input2.trim().parse().expect("Failed to input");

    println!("Input the value of c: ");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read c");
    let c:f32 = input3.trim().parse().expect("Failed to input");

    let d:f32 = (b * b) - (4.0 * a * c);
    let p = d.sqrt();
    if d > 0.0 {
        println!("There are 2 distinct roots in the equation");
        let x1 = (-b + p)/2.0 * a ;
         println!("The first root of the equation is {}", x1);
        let x2 = (-b - p)/2.0 * a;
        println!("The second root of the equation is {}", x2);

    } else if d == 0.0 {
        println!("There is only one distinct root.");
        let x = (-b + p)/2.0 * a;
        println!("The only root of the equation is {}", x);

    } else {
        println!("The roots are imaginary. They don't exist!");
    }


}
