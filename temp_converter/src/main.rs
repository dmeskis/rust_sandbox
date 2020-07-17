use std::io;

fn main() {
    let mut temperate_scale = String::new();

    println!("Welcome, would you like to convert a number to farenheit or celcius (f/c)");

    io::stdin()
        .read_line(&mut temperate_scale)
        .expect("Invalid!");

    println!("Great, you entered {}, now enter the temperature", temperate_scale);

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Invalid!");

    let temperature: u32 = temperature.trim().parse().expect("Please type a number!");

    if temperate_scale.trim() == "f" {
      let celcius = (temperature - 32) * 5/9;
      println!("The temperature is {} degrees celcius", celcius);
    } else if temperate_scale.trim() == "c" {
      let farenheit = (temperature * 9/5 ) + 32;
      println!("The temperature is {} degrees farenheit", farenheit);
    } else {
      println!("Something went wrong :(")
    }

}
