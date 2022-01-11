pub fn fib(fib_number: u64) -> u64 {
    if fib_number == 0 {
        return 0;
    }

    if fib_number < 2 {
        return 1;
    }

    let mut last_value: u64 = 1;
    let mut calculated_fib: u64 = 1;
    let mut summatory: u64;

    for _number in 2..fib_number {
        summatory = last_value + calculated_fib;
        last_value = calculated_fib;
        calculated_fib = summatory;
    }

    return calculated_fib;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_static_numbers() {
        let result_0 = fib(0);
        let result_1 = fib(1);
        let result_2 = fib(2);

        assert_eq!(result_0, 0);
        assert_eq!(result_1, 1);
        assert_eq!(result_2, 1);
    }

    #[test]
    fn test_numbers() {
        let result_0 = fib(3);
        let result_1 = fib(4);
        let result_2 = fib(5);
        let result_3 = fib(6);
        let result_4 = fib(7);

        assert_eq!(result_0, 2);
        assert_eq!(result_1, 3);
        assert_eq!(result_2, 5);
        assert_eq!(result_3, 8);
        assert_eq!(result_4, 13);
    }


    #[test]
    fn test_number_49() {
        assert_eq!(fib(49), 7778742049);
    }
}

