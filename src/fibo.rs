fn fibo(number: isize) -> isize {
    /*    if number <= 0 {
        return 0;
    } else if number == 1 {
        return 1;
    } else {
        let result = fibo(number - 1) + fibo(number - 2);
        return result;
    }*/

    if number.abs() < 2 {
        return number.abs();
    }

    let mut num1 = 0;
    let mut num2 = 1;
    for _ in 1..number.abs() {
        println!("num1 {} num2 {}", num1, num2);
        let num3 = num1 + num2;
        num1 = num2;
        num2 = num3;
        println!("- num1 {} num2 {}", num1, num2);
    }
    if number.is_negative() {
        -num2
    } else {
        num2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(fibo(15), 610);
        assert_eq!(fibo(-15), -610);
    }

    #[test]
    fn test2() {
        assert_eq!(fibo(1), 1);
        assert_eq!(fibo(-1), 1);
        assert_eq!(fibo(2), 1);
        assert_eq!(fibo(-2), -1);
        assert_eq!(fibo(0), 0);
    }
}
