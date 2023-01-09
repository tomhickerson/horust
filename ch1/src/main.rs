fn main() {
    println!("Hello, world!");
    // Chapter one - Recursion and Callbacks

    // Decimal to Binary Conversion 
    let mut st1:String = String::new();
    st1.push('a');
    // Factorials 
    let x:u64 = factorial(6);
    println!("{}", x);



}

fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n     => n * factorial(n - 1), 
    }
}
