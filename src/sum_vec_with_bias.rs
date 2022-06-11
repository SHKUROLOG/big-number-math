use crate::add_zeros::add_zeros;
use crate::sum::sum;

#[warn(dead_code)]
pub fn sum_vec_with_bias_old(source: &Vec<u32>) -> String {
    let mut acc: String = String::new();
    for (index, &el) in source.iter().enumerate() {
        println!("index, el - {} {}", index, el);
        let mut res: String = String::new();
        for _i in 0..index {
            res.push('0')
        }
        res.insert_str(0, el.to_string().as_str());
        println!("res - {}", res);
        acc = sum(&acc, &res);
        println!("acc - {}", acc);
    }
    acc
}

pub fn sum_vec_with_bias(source: &Vec<String>) -> String {
    source
        .iter()
        .enumerate()
        .map(|(index, el)| add_zeros(el, index))
        .reduce(|a, b| sum(&a, &b))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let source: Vec<String> = Vec::from([18, 12, 6])
            .iter()
            .map(|it| it.to_string())
            .collect();
        let expected: String = String::from("738");
        let at_result = sum_vec_with_bias(&source);
        assert_eq!(at_result, expected);
    }

    #[test]
    fn test2() {
        let source: Vec<String> = Vec::from([120, 18, 9])
            .iter()
            .map(|it| it.to_string())
            .collect();
        let expected: String = String::from("1200");
        let at_result = sum_vec_with_bias(&source);
        assert_eq!(at_result, expected);
    }

    #[test]
    fn test3() {
        let source: Vec<String> = Vec::from([9, 18, 920])
            .iter()
            .map(|it| it.to_string())
            .collect();
        let expected: String = String::from("92189");
        let at_result = sum_vec_with_bias(&source);
        assert_eq!(at_result, expected);
    }
}
