fn main() {
    let a1:i64 = 450000;
    let a2:i64 = 1500000;
    let a3:i64 = 750000;
    let a4:i64 = 2850000;
    let a5:i64 = 250000;
    let q1:i64 = 2;
    let q2:i64 = 1;
    let q3:i64 = 3;
    let q4:i64 = 3;
    let q5:i64 = 1;

    //to find the sum
    let s = a1 + a2 + a3 + a4 + a5;
    //to print the sum
    println! ("The sum of the amounts is {}", s);

    //to find the average
    let avg = s / q1 + q2 + q3 + q4 + q5;
    //to print the average
    println! ("The average of the sales records is {}", avg);


}