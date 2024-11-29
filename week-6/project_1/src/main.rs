use std::io;

fn main() {
    let p: i32 = 3200;
    let f: i32 = 3000;
    let a: i32 = 2500;
    let e: i32 = 2000;
    let w: i32 = 2500;
    let mut choice: String = "Y".to_string();
    let mut amount: i32 = 0;

    println!("Dear Esteemed Customer, Welcome to The Food Hut");
    println!("");
    println!("Here is the menu with the corresponding prices:");
    println!("");
    println!("Menu                                      Price");
    println!("");
    println!("Poundo Yam/Edinkaiko Soup                 N3,200");
    println!("Fried Rice & Chicken                      N3,000");
    println!("Amala & Ewedu Soup                        N2,500");
    println!("Eba & Egusi Soup                          N2,000");
    println!("White Rice & Stew                         N2,500");
    println!("");
    println!("For you to select your food order from the list, these are the input letters: ");
    println!("Poundo Yam/Edinkaiko Soup = P");
    println!("Fried Rice & Chicken = F");
    println!("Amala & Ewedu Soup = A");
    println!("Eba & Egusi Soup = E");
    println!("White Rice & Stew = W");
    println!("");

    while choice.trim() == "Y" {
        let mut input1 = String::new();
        println!("Please type in the letter for your order: ");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let order: String = input1.trim().to_string();

        let mut input2 = String::new();
        println!("How many portions of your order do you want? ");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let quantity: i32 = input2.trim().parse().expect("Not a valid integer for quantity");

        if order == "P" {
            println!("You have ordered {} portions of Poundo Yam/Edinkaiko Soup", quantity);
            amount += p * quantity;
        } else if order == "F" {
            println!("You have ordered {} portions of Fried Rice & Chicken", quantity);
            amount += f * quantity;
        } else if order == "A" {
            println!("You have ordered {} portions of Amala & Ewedu Soup", quantity);
            amount += a * quantity;
        } else if order == "E" {
            println!("You have ordered {} portions of Eba & Egusi Soup", quantity);
            amount += e * quantity;
        } else if order == "W" {
            println!("You have ordered {} portions of White Rice & Stew", quantity);
            amount += w * quantity;
        } else {
            println!("Your selection isn't in the format of the letter code or is not available on the menu.");
        }

        let mut input3 = String::new();
        println!("Is there anything else you would like to order? Please type Y for yes and N for no");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        choice = input3.trim().to_string();
    }

    println!(
        "Your total amount will be {}. If you would please move to the cashier to pay :) .", amount);
}
