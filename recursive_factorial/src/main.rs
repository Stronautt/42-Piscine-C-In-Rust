fn recursive_factorial(n: u128) -> Result<u128, String> {
    if n == 0 {
        return Ok(1);
    } else {
        n.checked_mul(recursive_factorial(n - 1)?)
            .ok_or("Factorial is too big for computation".to_owned())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_recursive_factorial() {
        assert_eq!(crate::recursive_factorial(0), Ok(1));
        assert_eq!(crate::recursive_factorial(1), Ok(1));
        assert_eq!(crate::recursive_factorial(2), Ok(2));
        assert_eq!(crate::recursive_factorial(3), Ok(6));
        assert_eq!(crate::recursive_factorial(4), Ok(24));
        assert_eq!(crate::recursive_factorial(5), Ok(120));
    }
}

fn main() {
    match std::env::args()
        .nth(1)
        .ok_or("Argument not set".to_owned())
        .and_then(|arg| arg.parse::<u8>().map_err(|err| err.to_string()))
        .and_then(|n| recursive_factorial(n as u128))
    {
        Ok(value) => println!("Factorial: {}", value),
        Err(err) => eprintln!("{}", err),
    };
}
