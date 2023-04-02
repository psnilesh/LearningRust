pub fn heap_sort<T>(seq: &mut Vec<T>)
where
    T: Ord,
{
    if seq.is_empty() {
        return;
    }

    make_heap(seq);

    let len = seq.len();
    for i in 0..len - 1 {
        let next_pos = len - 1 - i;
        seq.swap(0, next_pos);

        heapify(seq, 0, next_pos - 1);
    }
}

fn make_heap<T>(seq: &mut Vec<T>)
where
    T: Ord,
{
    if seq.len() <= 1 {
        return;
    }

    let last_index = seq.len() - 1;
    for idx in (0..last_index).rev() {
        heapify(seq, idx, last_index);
    }
}

fn heapify<T>(seq: &mut Vec<T>, start: usize, end: usize)
where
    T: Ord,
{
    if start == end {
        return;
    }

    let mut root = start;
    loop {
        let mut largest = root;
        let left = 2 * root + 1;
        let right = 2 * root + 2;

        if left <= end && seq[left] > seq[largest] {
            largest = left;
        }

        if right <= end && seq[right] > seq[largest] {
            largest = right;
        }

        if largest == root {
            break;
        }

        seq.swap(root, largest);
        root = largest;
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::sort::heap_sort;
    use rand::random;
    use std::vec;

    #[test]
    fn test_heap_sort_u32() {
        let mut random_array: Vec<u32> = (0..100).map(|_| random::<u32>() % 100).collect();
        let mut random_array_clone = random_array.clone();

        heap_sort(&mut random_array);
        random_array_clone.sort();

        assert_eq!(random_array, random_array_clone);
    }

    #[test]
    fn test_heap_sort_str() {
        let mut tokens = vec!["Hello", "World", "!", "How", "are", "you", "?"];

        let mut tokens_sorted_by_std_lib = tokens.clone();
        tokens_sorted_by_std_lib.sort();

        heap_sort(&mut tokens);
        assert_eq!(tokens_sorted_by_std_lib, tokens);
    }

    #[test]
    fn test_heap_sort_struct() {
        #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        let mut people = vec![
            Person {
                name: "John".to_string(),
                age: 32,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
            Person {
                name: "Alice".to_string(),
                age: 42,
            },
            Person {
                name: "Jasmine".to_string(),
                age: 35,
            },
        ];
        let mut people_sorted_by_std = people.clone();
        people_sorted_by_std.sort();

        heap_sort(&mut people);
        assert_eq!(people_sorted_by_std, people);
    }
}
