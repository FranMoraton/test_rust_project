pub fn to_celsius(farenheit: f64) -> f64 {
    ((farenheit - 32_f64) * 5_f64 / 9_f64 * 100_f64).ceil() / 100_f64
}

pub fn to_farenheit(celsius: f64) -> f64 {

    let mut result = celsius * 9_f64 / 5_f64;

    result = result + 32_f64;
    result = result * 100_f64;
    result = result.ceil();
    result = result / 100_f64;
    
/*     (((celsius * 9_f64 / 5_f64) + 32_f64) * 100_f64).ceil() / 100_f64; */

    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_celsius_to_fahrenheit() {
        assert_eq!(26.60, to_farenheit(-3.00));
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(37.40, to_farenheit(3.00));
    }

    #[test]
    fn test_0_celsius_to_fahrenheit() {
        assert_eq!(32_f64, to_farenheit(0.00));
    }

    #[test]
    fn test_negative_fahrenheit_to_celsius() {
        assert_eq!(-19.44, to_celsius(-3.00));
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(-16.11, to_celsius(3.00));
    }

    #[test]
    fn test_100_fahrenheit_to_celsius() {
        assert_eq!(37.78, to_celsius(100_f64));
    }
}
