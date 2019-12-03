fn safe_sqrt(n: f64) -> Result<f64, String> {
    let result = n.sqrt();
    if result.is_nan() {
        Err(format!("Can't compute square root for {}", n))
    } else {
        Ok(result)
    }
}

fn main() {
    match std::env::args()
        .nth(1)
        .ok_or("Argument not set".to_owned())
        .and_then(|arg| arg.parse::<f64>().map_err(|e| e.to_string()))
        .and_then(|n| safe_sqrt(n as f64))
    {
        Ok(value) => println!("Square root: {}", value),
        Err(err) => eprintln!("{}", err),
    };
}
