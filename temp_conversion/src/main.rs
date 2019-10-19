use std::io;

fn main() {
    loop {
        println!("Please input a temperature in Fahrenheit (or \"exit\" to continue): ");

        let mut fahr_input = String::new();
        io::stdin().read_line(&mut fahr_input)
            .expect("Failed to read line");
        
        let fahr_input: f64 = match fahr_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number, please enter a number.");
                continue;
            }
        };

        println!("Value in Celsius is {}", convert_fahr_to_celsius(fahr_input as f64));
    }
    
}

fn convert_fahr_to_celsius(fahrenheit: f64) -> f64 {
    let celsius = (5.0/9.0) * (fahrenheit - 32.0);
    celsius
}
