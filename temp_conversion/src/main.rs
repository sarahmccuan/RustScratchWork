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
// I want to break out the input handler from main
// Doing so requires making sure I have valid input
fn input_validation(user_input: &str) -> u8 {
    if user_input.trim() == "exit" {
        return 0; 
    } else {
        let user_input: f64 = match user_input.trim().parse() {
            Ok(num) => return 0,
            Err(_) => return 1,
        };
    }
}

#[test]
fn test_convert_fahr_to_celsius() {
    assert_eq!(convert_fahr_to_celsius(32.0), 0.0);
    assert_eq!(convert_fahr_to_celsius(212.0), 100.0);
}


#[test]
fn test_input_validation() { // put this in a nicer format pls
    let user_input = "1\n";
    assert_eq!(input_validation(user_input), 1);
    let user_input = "exit\n";
    assert_eq!(input_validation(user_input), 0);
    let user_input = "1.1\n";
    assert_eq!(input_validation(user_input), 0);
    let user_input = "0\n";
    assert_eq!(input_validation(user_input), 0);
}
  






































