use std::io;

fn main() {
    println!("Hello User! What operation would you like to perform today?");
    println!("Type the corresponding number to the operation you would like to perform");
    println!("1. Area of Trapezium \n2. Area of Rhombus \n3. Area of Parallelogram \n4. Area of cube \n5. Area of cylinder");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let choice:i32 = input1.trim().parse().expect("Not a valid number");

    if choice ==1 {
        println!("Type in the value of the base1 of the trapezium");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let a:f32 = input2.trim().parse().expect("Not a valid number");

        println!("Type in the value of the base2 of the trapezium");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Not a valid string");
        let b:f32 = input3.trim().parse().expect("Not a valid number");

        println!("Type in the value of the height of the trapezium");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Not a valid string");
        let h:f32 = input4.trim().parse().expect("Not a valid number");

        trapezium(a, b, h);

    } else if choice ==2 {

        println!("Type in the value of the diagonal1 of the rhombus");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let d1:f32 = input2.trim().parse().expect("Not a valid number");

        println!("Type in the value of the diagonal2 of the rhombus");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Not a valid string");
        let d2:f32 = input3.trim().parse().expect("Not a valid number");

        rhombus(d1, d2);

    }else if choice == 3 {

        println!("Type in the value of the base of the paralellogram");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let b:f32 = input2.trim().parse().expect("Not a valid number");

        println!("Type in the value of the base2 of the paralellogram");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Not a valid string");
        let a:f32 = input3.trim().parse().expect("Not a valid number");

        paralellogram(b, a);

    }else if choice ==4 {
        println!("Type in the value of the length of the cube");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let l:f32 = input2.trim().parse().expect("Not a valid number");

        cube(l);

    }else if choice ==5 {

        println!("Type in the value of the radius of the cylinder");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let r:f32 = input2.trim().parse().expect("Not a valid number");

        println!("Type in the value of the height of the cylinder");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Not a valid string");
        let h:f32 = input3.trim().parse().expect("Not a valid number");

        cylinder(r, h);

    }else {
        
        println!("You didn't input a value from the option above!");

    }
   
}

fn trapezium(a:f32, b:f32, h:f32) {
    
    let area = 0.5 * (a + b) * h ;
    println!("Area of trapezium = {}", area);

}

fn rhombus(d1:f32, d2:f32) {

    let area = 0.5 * d1 * d2 ;
    println!("Area of rhombus = {}", area);

}

fn paralellogram(b:f32, a:f32) {

    let area = b * a ;
    println!("Area of paralellogram = {}", area);

}

fn cube(l:f32) {
    let area = 6.0 * l ;
    println!("Area of cube = {}", area);

}

fn cylinder(r:f32, h:f32) {
    let volume = (22.0/7.0) * r * r * h;
    println!("Volume of cylinder = {}", volume);
}