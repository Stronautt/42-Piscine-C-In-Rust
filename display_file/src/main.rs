use std::io::Read;

fn display_file(file_name: &String) -> Result<String, String> {
    std::fs::File::open(&file_name)
        .map_err(|err| format!("{}: '{}'", err, file_name))
        .and_then(|file| {
            let mut buf_reader = std::io::BufReader::new(file);
            let mut contents = String::new();
            buf_reader
                .read_to_string(&mut contents)
                .map_err(|err| format!("{}: '{}'", file_name, err))?;
            Ok(contents)
        })
}

fn main() {
    for argument in &std::env::args().collect::<Vec<String>>() {
        match display_file(argument) {
            Ok(content) => print!("{}:\n{}", argument, content),
            Err(err) => eprintln!("{}", err),
        }
    }
}
