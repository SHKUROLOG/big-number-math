use crate::sum_vec_with_bias::sum_vec_with_bias;

pub fn multiply(a: &String, b: &String) -> String {
    let result: Vec<String> = b
        .chars()
        .rev()
        .map(|bit| {
            sum_vec_with_bias(
                &a.chars()
                    .rev()
                    .map(|ait| ait.to_digit(10).unwrap() * bit.to_digit(10).unwrap())
                    .map(|it| it.to_string())
                    .collect(),
            )
        })
        .collect();

    sum_vec_with_bias(&result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a: String = String::from("123");
        let b: String = String::from("456");
        let expected = String::from("56088");
        let at_result = multiply(&a, &b);
        assert_eq!(at_result, expected);
    }

    #[test]
    fn test2() {
        let a: String = String::from("234523452345234523452345234523452345");
        let b: String = String::from("432453456464567578567989238798928398");
        let expected = String::from(
            "101420477588699967170910567555104219651554819941053008126619185420193310",
        );
        let at_result = multiply(&a, &b);
        assert_eq!(at_result, expected);
    }
}
