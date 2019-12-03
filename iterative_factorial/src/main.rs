fn iterative_factorial(mut n: u8) -> Result<u128, String> {
    if n == 0 {
        return Ok(0);
    }
    let mut result: u128 = 1;
    while n > 0 {
        result = result
            .checked_mul(n as u128)
            .ok_or("Factorial is too big for computation".to_owned())?;
        n -= 1;
    }
    Ok(result)
}

fn main() {
    match std::env::args()
        .nth(1)
        .ok_or("Argument not set".to_owned())
        .and_then(|arg| arg.parse::<u8>().map_err(|e| e.to_string()))
        .and_then(|n| iterative_factorial(n))
    {
        Ok(value) => println!("Factorial: {}", value),
        Err(err) => eprintln!("{}", err),
    };
}
