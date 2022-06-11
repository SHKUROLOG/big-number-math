use crate::get_char_at_index::get_char_at_index;

pub fn sum(a: &String, b: &String) -> String {
    let a_len = a.len();
    let b_len = b.len();
    let mut in_mind = 0u32;
    let mut main_result = String::new();

    let max_len = if b_len > a_len { b_len } else { a_len };
    let a_rev = a.chars().rev().collect();
    let b_rev = b.chars().rev().collect();

    for index in 0..max_len {
        let a_number = get_char_at_index(&a_rev, index).unwrap_or(0);
        let b_number = get_char_at_index(&b_rev, index).unwrap_or(0);
        let ab_num: u32 = a_number + b_number;
        let to_parse = ab_num % 10;
        let mut to_save = (ab_num - to_parse) / 10;
        let mut number_result = to_parse + in_mind;
        if number_result == 10 {
            to_save += 1;
            number_result = 0;
        }
        let result = char::from_digit(number_result, 10).unwrap();
        main_result.insert(0, result);
        in_mind = to_save;
    }
    if in_mind > 0 {
        main_result.insert(0, char::from_digit(in_mind, 10).unwrap());
    }
    main_result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        let a: String = String::from("123");
        let b: String = String::from("456");
        let expected = String::from("579");
        let at_result = sum(&a, &b);
        assert_eq!(at_result, expected);
    }

    #[test]
    fn check_overflow() {
        let a: String = String::from("543");
        let b: String = String::from("567");
        let expected = String::from("1110");
        let at_result = sum(&a, &b);
        assert_eq!(at_result, expected);
    }

    #[test]
    fn check_overflow2() {
        let a: String = String::from("929");
        let b: String = String::from("567");
        let expected = String::from("1496");
        let at_result = sum(&a, &b);
        assert_eq!(at_result, expected);
    }

    #[test]
    fn check_diff_len() {
        let a: String = String::from("11929");
        let b: String = String::from("67");
        let expected = String::from("11996");
        let at_result = sum(&a, &b);
        assert_eq!(at_result, expected);
    }

    #[test]
    fn check_big_num() {
        let a: String = String::from("234523452345234523452345234523452345");
        let b: String = String::from("432453456464567578567989238798928398");
        let expected = String::from("666976908809802102020334473322380743");
        let at_result = sum(&a, &b);
        assert_eq!(at_result, expected);
    }

    #[test]
    fn check_fucking_big_num() {
        let a: String = String::from("234523452345234523452345234523452345");
        let b:String = String::from("234523452345234523452345234523452345234523452345234523452345234523452345234523452345234523452345234523452345234523452345234523452345234523452345432453456464567578567989238798928398");
        let expected = String::from("234523452345234523452345234523452345234523452345234523452345234523452345234523452345234523452345234523452345234523452345234523452345234523452345666976908809802102020334473322380743");
        let at_result = sum(&a, &b);
        assert_eq!(at_result, expected);
    }
}
