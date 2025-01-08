// declare a structure called Laptops
struct Laptops {
    price:u32,
    quantity:u32
}

fn main() {
    //initialize all the structures
    let hp = Laptops {
        price:650000,
        quantity:10
    };
    let ibm = Laptops {
        price:755000,
        quantity:6
    };
    let toshiba = Laptops {
        price:550000,
        quantity:10
    };
    let dell = Laptops {
        price:850000,
        quantity:4
    };

    let total_cost = (hp.price * 3) + (ibm.price * 3) + (toshiba.price * 3) + (dell.price * 3);
    println!("The total cost if the customer buys three from each brand is {} \n", total_cost);

    println!("These are the numbers of the remaining laptops in stock:");
    println!("HP laptops: {}", hp.quantity - 3);
    println!("IBM laptops: {}", ibm.quantity - 3);
    println!("Toshiba laptops: {}", toshiba.quantity - 3);
    println!("Dell laptops: {} \n\n\n", dell.quantity - 3);

    println!("Ce fini! Your's truly - Kachi.");
    
}