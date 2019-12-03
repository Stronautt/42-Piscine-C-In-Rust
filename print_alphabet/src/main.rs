fn main() {
    let mut symb = 'a';
    while symb <= 'z' {
        print!("{}", symb);
        symb = (symb as u8 + 1) as char;
    }
}
