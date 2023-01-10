fn main() {
    println!("Hello, world!");
    // Chapter one - Recursion and Callbacks

    // Decimal to Binary Conversion 
    let mut st1:String = String::new();
    st1.push('a');
    // Factorials 
    let x:u64 = factorial(6);
    println!("factorial: {}", x);

    let y:i32 = to_binary(16);
    println!("to binary: {}", y);
}

fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n     => n * factorial(n - 1), 
    }
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
