fn div_mod(x: i32, y: i32) -> Result<(i32, i32), String> {
    let div = x
        .checked_div(y)
        .ok_or(format!("Division by {} is not possible", y))?;
    let module = x
        .checked_rem(y)
        .ok_or(format!("Modulos from {} is not possible", y))?;
    Ok((div, module))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_div_mod() {
        assert_eq!(crate::div_mod(5, 2), Ok((2, 1)));
    }

    #[test]
    fn test_div_mod_zero() {
        assert!(crate::div_mod(5, 0).is_err());
        assert!(crate::div_mod(0, 5).is_ok());
    }
}

fn get_argument(pos: usize) -> Result<i32, String> {
    std::env::args()
        .nth(pos)
        .ok_or(format!("Argument #{} was not specified", pos))
        .and_then(|num| num.to_owned().parse::<i32>().map_err(|err| err.to_string()))
}

fn main() {
    match get_argument(1).and_then(|x| div_mod(x, get_argument(2)?)) {
        Ok((div, module)) => println!("Div: {}, Mod: {}", div, module),
        Err(err) => eprintln!("{}", err),
    }
}
