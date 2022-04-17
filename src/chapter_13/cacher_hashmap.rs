use std::{collections::HashMap, hash::Hash};

pub struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Hash + Eq + Copy,
    V: Copy,
{
    calculation: T,
    value: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Hash + Eq + Copy,
    V: Copy,
{
    pub fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: K) -> V {
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
