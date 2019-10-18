fn main() {
    let x = five();
    println!("The value of x is {}", x); // main is not a fruitful function
}

fn five() -> i32 { // this is a fruitful function
    5 // returns value of last line by default
}
