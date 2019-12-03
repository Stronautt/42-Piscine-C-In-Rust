fn foreach<T, F>(tab: &Vec<T>, mut func: F)
where
    F: FnMut(&T),
{
    for num in tab {
        func(num);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_div_mod() {
        let tab = vec![1, 2, 3, 5, 8];
        let mut res: i32 = 0;

        crate::foreach(&tab, |num: &i32| res += *num);
        assert_eq!(res, 19);
    }
}

fn main() {
    foreach(&vec![1, 21, 42], |num| println!("{}", num));
}
