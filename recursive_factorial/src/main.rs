fn iterative_factorial(n: u128) -> Result<u128, String> {
    if n == 0 {
        return Ok(1);
    } else {
        n.checked_mul(iterative_factorial(n - 1)?)
            .ok_or("Factorial is too big for computation".to_owned())
    }
}

fn main() {
    match std::env::args()
        .nth(1)
        .ok_or("Argument not set".to_owned())
        .and_then(|arg| arg.parse::<u8>().map_err(|err| err.to_string()))
        .and_then(|n| iterative_factorial(n as u128))
    {
        Ok(value) => println!("Factorial: {}", value),
        Err(err) => eprintln!("{}", err),
    };
}
