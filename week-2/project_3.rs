fn main() {
    let p: f64 = 510000.0; 
    let t: i32 = 3;
    let r: f64 = 5.0; 

    // To find the depreciated amount
    let da = p * (1.0 - (r / 100.0)).powi(t);
    
    // To print the result of the depreciated amount
    println!("The Depreciated Amount is {}", da);
}
