use std::fmt::Debug;

pub fn main() {
    let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
    // groups:     |->|---->|->|->|--->|----------->|--->|

    // data goes out of scope once iterator is created and iterators transfor itself into mutable
    // let mut iterator_on_data = Groups::new(data.clone());
    // println!("item1 returned {:?}", iterator_on_data);
    //
    // let item = iterator_on_data.next();
    // println!("item1 returned {:?}", item.unwrap());
    //
    // println!("item1 iterator remains {:?}", iterator_on_data);
    //
    // let item2 =iterator_on_data.next();
    // println!("item2 returned {:?}", item2.unwrap());

    assert_eq!(
        Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![4],
            vec![1, 1],
            vec![2],
            vec![1],
            vec![3, 3],
            vec![-2, -2, -2],
            vec![5, 5],
        ]
    );

    let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
    // groups:      |->|---->|---->|----|->|----->|->|
    assert_eq!(
        Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![1],
            vec![2, 2],
            vec![1, 1],
            vec![2, 2],
            vec![3],
            vec![4, 4],
            vec![3],
        ]
    )
}

#[derive(Debug)]
struct Groups<T> {
    inner: Vec<T>,
}

// groups functions declaration
impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self {
        Groups { inner }
    }
}

impl<T: PartialEq + Debug> Iterator for Groups<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // if the inner vector is empty, we are done
        if self.inner.is_empty() {
            return None;
        }

        // lets check the span of equal items
        let mut cursor = 1;
        let first = &self.inner[0];
        for element in &self.inner[1..] {
            if element == first {
                cursor += 1;
            } else {
                break;
            }
        }
        println!("inner call{:?}", self.inner);
        // we use the `Vec::drain` to extract items up until the cursor
        let items = self.inner.drain(0..cursor).collect();
        println!("items drained {:?}", items);
        // return the extracted items
        Some(items)
    }
}
