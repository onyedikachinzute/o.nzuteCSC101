use std::io::Write;

fn main() {
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let nonalcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    println!("This makes a portfolio file for Nigerian Brewery Limited.");
    println!("File creation beginning...");

    let mut file = std::fs::File::create("portfolio.txt").expect("create failed");

    file.write_all("This is the portfolio for Nigerian Brewery Limited\n\n".as_bytes()).expect("write failed");
    file.write_all(format!("{:<15} {:<15} {:<15}\n", "Lager", "Stout", "Non-Alcoholic").as_bytes()).expect("write failed");//format adds the padding between the values and makes it such that it mustn't be a string value.

    let max_rows = std::cmp::max(lager.len(), std::cmp::max(stout.len(), nonalcoholic.len())); //this finds the greatest number of rows my finding the max length of all the vectors.

    for i in 0..max_rows { //using a for loop, we write all the values in an easy way.
        let lager_entry = lager.get(i).unwrap_or(&"");
        let stout_entry = stout.get(i).unwrap_or(&"");
        let nonalcoholic_entry = nonalcoholic.get(i).unwrap_or(&"");//this checks if the value is there and extracts it. If it isn't there, it prints an empty string instead.

        file.write_all(format!("{:<15} {:<15} {:<15}\n",lager_entry, stout_entry, nonalcoholic_entry).as_bytes()).expect("write failed");
    }

    println!("File written successfully.");
    println!("To be honest. Quite Easily Done. In 28 lines of code. :D")
}
