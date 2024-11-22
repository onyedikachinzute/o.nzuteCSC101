fn main() {
    let p: f64 = 520000000.0; 
    let t: i32 = 5;
    let r: f64 = 10.0; 

    // Calculate amount using the formula A = P * (1 + r/n)^(nt)
    
    let a = p * (1.0 + (r / 100.0)).powi(t);
    let ci = a - p;
    
    //The result of the compound interest
    println!("The Compound Interest is {}", ci);
}
