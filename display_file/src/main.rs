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

#[cfg(test)]
mod tests {
    use std::io::Write;
    extern crate tempfile;

    #[test]
    fn test_display_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("existing_file.txt");
        let fake_content = String::from("Hello World");
        let mut fake_file = std::fs::File::create(file_path).unwrap();
        write!(fake_file, "{}", fake_content).unwrap();

        std::env::set_current_dir(&dir).unwrap();
        assert_eq!(
            crate::display_file(&"existing_file.txt".to_owned()),
            Ok(fake_content)
        );
    }

    #[test]
    fn test_display_non_existing_file() {
        let dir = tempfile::tempdir().unwrap();

        std::env::set_current_dir(&dir).unwrap();
        assert!(crate::display_file(&"non_existing_file.txt".to_owned()).is_err());
    }
}

fn main() {
    for argument in &std::env::args().collect::<Vec<String>>()[1..] {
        match display_file(argument) {
            Ok(content) => print!("{}:\n{}", argument, content),
            Err(err) => eprintln!("{}", err),
        }
    }
}
