
pub fn get_char_at_index (source: &String, index: usize) -> Option<u32> {
    if let Some(b_char) = source.chars().nth(index) {
        if let Ok(b_number) = String::from(b_char).parse::<u32>() {
            return Some(b_number)
        };
    };
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_index (){
        let source: String = String::from("11929");
        let index: usize = 2;
        let expected = Some(9u32);
        let result = get_char_at_index(&source, index);
        assert_eq!(result, expected);
    }

    #[test]
    fn check_index_none (){
        let source: String = String::from("11929");
        let index: usize = 10;
        let expected = None;
        let result = get_char_at_index(&source, index);
        assert_eq!(result, expected);
    }
}