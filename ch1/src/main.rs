use std::ops::Mul;

fn main() {
    println!("Hello, world!");
    // Chapter one - Recursion and Callbacks

    // Decimal to Binary Conversion 
    let y:i32 = to_binary(16);
    println!("to binary: {}", y);
    let z:i32 = to_binary(32);
    println!("to binary: {}", z);
    // Factorials 
    let x:u64 = factorial(6);
    println!("factorial: {}", x);

    let x2:u64 = factorial2(6);
    println!("factorial 2: {}", x2);
    // Tower of Hanoi 

    // Data in a Hierarchy 

    
}

fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n     => n * factorial(n - 1), 
    }
}

fn factorial2(n: u64) -> u64 {
    (1..n+1).fold(1, Mul::mul)
}

fn to_binary(mut decimal: i32) -> i32 {
    if decimal == 0 {
        decimal
    } else {
        let mut bits = String::new();

        while decimal > 0 {
            if decimal % 2 == 0 {
                bits.push_str("0");
            } else {
                bits.push_str("1");
            }

            decimal /= 2;
        }

        // reverse the bits
        match bits.chars().rev().collect::<String>().parse() {
            Ok(num) => num,
            Err(_e) => panic!("Something went wrong"),
        }
    }
}
