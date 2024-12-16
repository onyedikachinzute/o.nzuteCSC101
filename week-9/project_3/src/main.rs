use std::io::Write;

fn main() {
    let sn = vec!["1", "2", "3", "4", "5"];
    let names = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministries = ["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let geo_zones = ["South West", "North East", "South South", "South West", "South East"];

    println!("This will create a file showing all the details of all the convicted ministers in the country.");
    
    let mut file = std::fs::File::create("Convicted_Ministers.txt").expect("create failed");
    file.write_all("Convicted Ministers across the country!\n\n".as_bytes()).expect("Write failed");
    file.write_all(format!("{:<10} {:<30} {:<20} {:<20}\n","S/N", "Name of Commisioner", "Ministry", "Geopolitical Zone").as_bytes()).expect("Write failed");

    let max = names.len();

    for i in 0..max {
        let sn_entry = sn.get(i).unwrap_or(&"Unknown");
        let names_entry = names.get(i).unwrap_or(&"Unknown");
        let ministries_entry = ministries.get(i).unwrap_or(&"Unknown");
        let geo_zones_entry = geo_zones.get(i).unwrap_or(&"Unknown");

        file.write_all(format!("{:<10} {:<30} {:<20} {:<20}\n",sn_entry, names_entry, ministries_entry, geo_zones_entry).as_bytes()).expect("write failed");

    }

    println!("The file has been created successfully!");
    println!("QED");

}
