pub fn fib(fib_number: u64) -> u64 {
    match fib_number {
        0 => 0,
        1|2 => 1,
        _ => {
            let mut last_value: u64 = 1;
            let mut calculated_fib: u64 = 1;
            let mut summatory: u64;
        
            for _number in 2..fib_number {
                summatory = last_value + calculated_fib;
                last_value = calculated_fib;
                calculated_fib = summatory;
            }
        
            calculated_fib
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_static_numbers() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
    }

    #[test]
    fn test_numbers() {
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(6), 8);
        assert_eq!(fib(7), 13);
    }


    #[test]
    fn test_number_49() {
        assert_eq!(fib(49), 7778742049);
    }
}

