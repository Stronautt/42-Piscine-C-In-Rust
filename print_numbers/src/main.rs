fn main() {
    let mut symb = '0';
    while symb <= '9' {
        print!("{}", symb);
        symb = (symb as u8 + 1) as char;
    }
    print!("\n");
}
