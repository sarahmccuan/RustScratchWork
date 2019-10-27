fn main() {
    let v = vec![1, 2, 3, 4, 5,]; 

    let third = &v[2];

    println!("third value of the vector is {}", third);

    // this returns an option so pretty printing kind of drops everything
    // if you put a number in range it is okay 
    // if you put a number out of range it returns None 
    // which is technically fine but probably not what you intend
    println!("the third value of the vector is {:?}", v.get(7));

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Borrowing and ownership in vecs
    let mut v2 = vec![1, 2, 3, 4, 5,];


    v2.push(6);

    {
        println!("Vec v2 is {:?}", v2);
        let mut v3 = v2;
        println!("Vec v3 is {:?}", v3);
    
        v3.push(7);
        println!("Vec v3 = {:?}", v3);
    }
    // This is borrow after modification of original v2, might have changed. 
    // println!("Vec v2 = {:?}", v2);
    

    let number: i32 = 10;
    println!("number is {}", number);
    println!("number is {}", number);
    let another_number = number + 5; // this will copy b/c i32's are smol
    println!("another_number is {}", another_number);

    let string = String::from("hello");
    println!("string is {}", string);
    // here we need to use a reference... not copied?
    let another_string = string + &String::from(" world");
    println!("another_string is {}", another_string);
    // following line breaks b/c of borrow to create another_string
    // println!("string is still {}", string);


}
