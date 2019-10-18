use std::io;

fn main() {
    let mut user_input = String::new();

    loop { // I need to have expect here somehow
        user_input = io::stdin().read_line(&mut user_input)
            .unwrap();
        if user_input == "exit" {
            break;
        } else {
            println!("User input is {}", user_input);
        }
    }
    
    println!("Swapped value is {}", convert_fahr_to_celsius(1.1));
}

fn convert_fahr_to_celsius(fahrenheit: f64) -> f64 {
    5.0
}
