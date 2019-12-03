fn main() {
    let mut buffer = String::new();
    match std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())
        .and_then(|_bytes| {
            let num = buffer.trim().parse::<i128>().map_err(|e| e.to_string())?;
            if num.is_negative() {
                Ok('N')
            } else {
                Ok('P')
            }
        }) {
        Ok(sign) => println!("{}", sign),
        Err(error) => eprintln!("{}", error),
    }
}
