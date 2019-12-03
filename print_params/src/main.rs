fn main() {
    for argument in &std::env::args().collect::<Vec<String>>()[1..] {
        println!("{}", argument);
    }
}
