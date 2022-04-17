use std::collections::HashMap;

pub struct Cacher<T>
where
    T: Fn(u64) -> u64,
{
    calculation: T,
    value: HashMap<u64, u64>,
}

impl<T> Cacher<T>
where
    T: Fn(u64) -> u64,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u64) -> u64 {
        *self.value.entry(arg).or_insert((self.calculation)(arg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
