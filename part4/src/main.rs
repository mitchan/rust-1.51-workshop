fn main() {
    let mut city_names = vec!["Pythonia", "Javasburg", "C by the Sea", "Rustville"];

    let last_city = city_names.pop();

    match last_city {
        Some(city) => {
            if city.starts_with("R") {
                println!("“{}” starts with an R!", city);
            } else {
                println!("“{}” doesn't start with R", city);
            }

            city_names.push(city);
        }

        None => {
            println!("No city found");
        }
    }

    println!("Here is the full list of cities:");

    for city in city_names.iter() {
        println!("City: {}", city);
    }
}
