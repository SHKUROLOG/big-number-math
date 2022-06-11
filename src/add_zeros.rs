use std::ops::Add;

pub fn add_zeros(source: &String, count: usize) -> String {
    source.clone().add(String::from("0").repeat(count).as_str())
}

#[cfg(test)]
mod tests {
    use super::add_zeros;

    #[test]
    fn test() {
        let source: String = String::from("123");
        let count: usize = 5;
        let expected = String::from("12300000");
        let at_result = add_zeros(&source, count);
        assert_eq!(at_result, expected);
    }
}
