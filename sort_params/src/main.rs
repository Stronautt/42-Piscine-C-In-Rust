fn main() {
    let arguments = &mut std::env::args().collect::<Vec<String>>()[1..];
    arguments.sort();
    for argument in arguments {
        println!("{}", argument);
    }
}
