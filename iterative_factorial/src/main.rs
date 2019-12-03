fn iterative_factorial(mut n: u8) -> Result<u128, String> {
    if n == 0 {
        return Ok(1);
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_iterative_factorial() {
        assert_eq!(crate::iterative_factorial(0), Ok(1));
        assert_eq!(crate::iterative_factorial(1), Ok(1));
        assert_eq!(crate::iterative_factorial(2), Ok(2));
        assert_eq!(crate::iterative_factorial(3), Ok(6));
        assert_eq!(crate::iterative_factorial(4), Ok(24));
        assert_eq!(crate::iterative_factorial(5), Ok(120));
    }
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
