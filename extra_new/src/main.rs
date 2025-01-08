use std::io;

// declare a structure called Laptops
struct Laptops {
    price:u32,
    quantity:u32,
    name:String
}

fn main() {
    //initialize all the structures
    let mut hp = Laptops {
        name:String::from("HP"),
        price:650000,
        quantity:10
    };
    let mut ibm = Laptops {
        name:String::from("IBM"),
        price:755000,
        quantity:6
    };
    let mut toshiba = Laptops {
        name:String::from("Toshiba"),
        price:550000,
        quantity:10
    };
    let mut dell = Laptops {
        name:String::from("Dell"),
        price:850000,
        quantity:4
    };

    let mut total_amount:u32 = 0;
    let mut cont = "y".to_string();

    while cont == "y" {
        let mut input1 = String::new();
        println!("Which laptop do you want to buy? ");
        println!("1. Hp \n2. IBM \n3. Toshiba \n4. Dell \n");
        println!("Please type in the name of the laptop, not the number next to it. \n");
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let choice = input1.trim().to_string().to_lowercase();

        let mut input2 = String::new();
        println!("How many laptops do you want to buy?");
        io::stdin().read_line(&mut input2).expect("Not a valid input");
        let number:u32 = input2.trim().parse().expect("Not a valid number");

        if choice == "hp" {
            if number > hp.quantity {
                println!("This is not possible as there are no more laptops in stock.");
            }else {
                total_amount = total_amount + (hp.price * number);
                hp.quantity = hp.quantity - number;
            }  
    }   else if choice == "ibm" {
            if number > ibm.quantity {
                println!("This is not possible as there are no more laptops in stock.");
            }else {
                total_amount = total_amount + (ibm.price * number);
                ibm.quantity = ibm.quantity - number;
            }
    }   else if choice == "toshiba" {
            if number > toshiba.quantity {
                println!("This is not possible as there are no more laptops in stock.");
            }else {
                total_amount = total_amount + (toshiba.price * number);
                toshiba.quantity = toshiba.quantity - number;
            }    
    }   else if choice == "dell" {
            if number > dell.quantity {
                println!("This is not possible as there are no more laptops in stock.");
            }else {
                total_amount = total_amount + (dell.price * number);
                dell.quantity = dell.quantity - number;
            }
    }   else {
            println!("Your selection isn't one of them.");
    }

        println!("Your total amount right now is {}", total_amount);
        let mut input3 = String::new();
        println!("Do you wish to make another purchase? Y/N ");
        io::stdin().read_line(&mut input3).expect("Not a valid input");
        cont = input3.trim().to_string().to_lowercase();
    }
    
    println!("Your total amount at the end is {}", total_amount);
    println!("Thank You!");
    println!("This is all that is left of every brand");
    display(hp);
    display(ibm);
    display(toshiba);
    display(dell);


    
}

fn display(lap:Laptops) {
    println!("The brand is {}, There are {} units left. ", lap.name, lap.quantity);
}