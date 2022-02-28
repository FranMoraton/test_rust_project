use std::collections::HashMap;

pub fn median(list: &mut Vec<u32>) -> f32 {
    if list.len() == 0 {
        return 0f32;
    }

    list.sort();

    match list.len() % 2 {
        0 => return (list.get((list.len() / 2) - 1).unwrap().to_owned() as f32 + list.get(list.len() / 2).unwrap().to_owned() as f32) / 2f32,
        _ => return list.get(list.len() / 2).unwrap().to_owned() as f32,
    }
}

pub fn mode(list: Vec<u32>) -> u32 {
    let mut counting_values: HashMap<u32, u32> = HashMap::new();

    for item in list {
        let item_value_counter = counting_values.entry(item).or_insert(0);
        *item_value_counter += 1;
    }

    let mut mode_in_list = 0;
    let mut mode_in_list_value = 0;

    for (key, value) in counting_values.clone() {
        if value > mode_in_list_value {
            mode_in_list_value = value; 
            mode_in_list = key;
        }
    }

    counting_values
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers");

    return mode_in_list;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn return_zero_when_vector_is_empty() {
        let mut test_vector: Vec<u32> = vec![];
        assert_eq!(median(&mut test_vector), 0f32);
    }

    #[test]
    fn return_median_when_vector_is_odd() {
        let mut test_vector: Vec<u32> = vec![1, 2, 3];
        assert_eq!(median(&mut test_vector), 2f32);
    }

    #[test]
    fn return_median_when_vector_is_even_must_be_median_between_central_numbers() {
        let mut test_vector: Vec<u32> = vec![1, 3, 3, 2];
        assert_eq!(median(&mut test_vector), 2.5);

        let mut test_vector: Vec<u32> = vec![1, 3];
        assert_eq!(median(&mut test_vector), 2f32);
    }

    #[test]
    fn return_mode_zero_when_vector_is_empty() {
        let test_vector: Vec<u32> = vec![];
        assert_eq!(mode(test_vector), 0);
    }

    #[test]
    fn return_mode_when_vector_contains_numbers() {
        let test_vector: Vec<u32> = vec![1, 2, 3, 2];

        assert_eq!(mode(test_vector), 2);
    }
}