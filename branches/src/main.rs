fn main() {
    let number: i32 = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // using conditional in declaration
    let condition: bool = true;

    let second_number: i32 = if condition {
        5
    } else {
       6 
    };

    println!("The value of number is {}", second_number);
}
