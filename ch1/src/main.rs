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

fn binary(n: u64) -> String {
    let mut ret:String = String:new();
    match n {
        0    => ret.push('0'),
        1    => ret.push('1'),
        _    => binary(n % 2),
    }

}
