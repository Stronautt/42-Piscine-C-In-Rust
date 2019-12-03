fn foreach<T>(tab: Vec<T>, func: fn(&T)) {
    for num in &tab {
        func(num);
    }
}

fn main() {
    foreach(vec![1, 21, 42], |num| println!("{}", num));
}
