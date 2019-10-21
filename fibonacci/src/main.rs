// Calculates the nth Fibonacci number given n

fn main() {
    println!("hello, world");
    // Get "n" from user and validate until legit n given
    // calculate nth fibonacci number (break this out as func w/ test)
    // print to user and end program
    println!("first number is {}", calculate_nth_fibonacci_number(1));
    println!("second number is {}", calculate_nth_fibonacci_number(2));
    println!("third number is {}", calculate_nth_fibonacci_number(3));
    println!("fourth number is {}", calculate_nth_fibonacci_number(4));
    println!("fifth number is {}", calculate_nth_fibonacci_number(5));
    println!("sixth number is {}", calculate_nth_fibonacci_number(6));
    println!("seventh number is {}", calculate_nth_fibonacci_number(7));
    println!("eighth number is {}", calculate_nth_fibonacci_number(8));
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

#[test]
fn test_calculate_nth_fibonacci_number(){
    // test valid inputs
    let first_fibonacci_number = 1;
    assert_eq!(calculate_nth_fibonacci_number(1), first_fibonacci_number);
    let seventh_fibonacci_number = 13;
    assert_eq!(calculate_nth_fibonacci_number(13), seventh_fibonacci_number);

    // test invalid inputs. if n <= 0, return -1 
    let zeroth_fibonacci_number = -1; 
    assert_eq!(calculate_nth_fibonacci_number(0), zeroth_fibonacci_number);
    let minus_seventh_fibonacci_number = -1;
    assert_eq!(calculate_nth_fibonacci_number(-7), minus_seventh_fibonacci_number);
}
