enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind, // kind is one of the possibilities defined in IpAddrKind
    address: String,  // just saying what it is 
}


fn main () {
    // here we are setting a specific instance of an Ip
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
        };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };


    let four = IpAddrKind::V4; // 
    let six = IpAddrKind::V6;

    let five = Some(5);
    let five_plus_one = plus_one(five);
    let none = plus_one(None);

    println!("Value of five is {:?}", five);
    println!("Value of five_plus_one is {:?}", five_plus_one);
    println!("Value of none is {:?}", none);
    
    
}

fn route(ip_ind: IpAddrKind) { // function takes one of the data instances 

}

// take an Option<i32>. If has value, add one, else do nothing and return None
fn plus_one(original_number: Option<i32>) -> Option<i32> {
    match original_number {
        None => None,
        Some(i) => Some(i + 1), // where the heck did i come from? 
    }
}
