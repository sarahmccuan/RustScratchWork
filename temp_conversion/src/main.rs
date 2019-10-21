use std::io;

fn main() {
    loop {
        println!("Please input a temperature in Fahrenheit (or \"exit\" to continue): ");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line");

        if user_input.trim() == "exit" {
            println!("Thanks, bye!");
            break;
        }
        
        let user_input: f64 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number, please enter a number.");
                continue;
            }
        };

        println!("Value in Celsius is {:.2}", convert_fahr_to_celsius(user_input as f64));
    }
    
}

fn convert_fahr_to_celsius(fahrenheit: f64) -> f64 {
    let celsius = (5.0/9.0) * (fahrenheit - 32.0);
    celsius
}

#[test]
fn test_convert_fahr_to_celsius() {
    assert_eq!(convert_fahr_to_celsius(32.0), 0.0);
    assert_eq!(convert_fahr_to_celsius(212.0), 100.0);
}







































