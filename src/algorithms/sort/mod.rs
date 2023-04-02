use std::fmt::Debug;

mod heap_sort;

pub fn heap_sort<T>(seq: &mut Vec<T>)
where
    T: Ord + Debug,
{
    heap_sort::heap_sort(seq)
}
