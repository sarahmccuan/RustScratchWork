// Calculates the nth Fibonacci number given n
use std::io;

fn main() {
    println!("hello, world");
    // Get "n" from user and validate until legit n given
    loop {
        println!("Enter a number \"n\" greater than 0");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line");
       
        let n: i64 = match user_input.trim().parse::<i64>() {
            Ok(num) => {
                num;
                break;
            }
            Err(_) => {
                println!("Not a valid number for n!");
                continue;
            }
        };
    }

    println!("value entered is {}", n);
    // calculate nth fibonacci number (break this out as func w/ test)
    // print to user and end program
}


fn calculate_nth_fibonacci_number(n: i64) -> i64 {
    if n < 1 {
        -1
    }
    else if n == 1 {
        1
    }
    else if n == 2 {
        1
    }
    else {
        calculate_nth_fibonacci_number(n - 1) + calculate_nth_fibonacci_number(n-2)
    }
}

fn validate_user_input(input_string: &str) {

}







#[test]
fn test_calculate_nth_fibonacci_number(){
    // test valid inputs
    let first_fibonacci_number = 1;
    assert_eq!(calculate_nth_fibonacci_number(1), first_fibonacci_number, 
               "first fibonacci number was not calculated correctly");
    let seventh_fibonacci_number = 13;
    assert_eq!(calculate_nth_fibonacci_number(7), seventh_fibonacci_number,
                "seventh fibonacci number was not calculated correctly");

    // test invalid inputs. if n <= 0, return -1 
    let zeroth_fibonacci_number = -1; 
    assert_eq!(calculate_nth_fibonacci_number(0), zeroth_fibonacci_number,
               "zeroth fibonacci number was not calculated correctly");
    let minus_seventh_fibonacci_number = -1;
    assert_eq!(calculate_nth_fibonacci_number(-7), minus_seventh_fibonacci_number,
               "minus seventh fibonacci was not calculated correctly");
}

#[test]
fn test_validate_user_input(){
    // test falid inputs
    let valid_input = "5\n"; // should be a positive integer as string w/ return
    assert_eq!(validate_user_input(valid_input), valid_input); // should return cast input if valid 

    // test invalid inputs
    let invalid_input_response = -1; // should always return -1 for bad input
    let invalid_negative_input = "-5\n";
    assert_eq!(validate_user_input(invalid_negative_input), invalid_input_response);
    let invalid_input_response = "hello\n";
    assert_eq!(validate_user_input(invalid_negative_input), invalid_input_response);
}






















