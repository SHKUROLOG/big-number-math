use crate::multiply::multiply;

pub fn factorial(base: u32) -> String {
    let mut result = String::from("1");
    for i in 1..base + 1 {
        result = multiply(&result, &i.to_string());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::factorial;

    #[test]
    fn test1() {
        let base = 15u32;
        let expected = String::from("1307674368000");
        let at_result = factorial(base);
        assert_eq!(at_result, expected);
    }

    #[test]
    fn test2() {
        let base = 100u32;
        let expected = String::from("93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000");
        let at_result = factorial(base);
        assert_eq!(at_result, expected);
    }
}
