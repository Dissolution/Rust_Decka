use std::cmp::Ordering;
use std::fmt::*;

pub fn get_two_mut<T>(slice: &mut [T], index1: usize, index2: usize) -> Option<(&mut T, &mut T)> {
    if index1 == index2 {
        None
    } else if index1 < index2 {
        let (start, end) = slice.split_at_mut(index2);
        Some((&mut start[index1], &mut end[0]))
    } else {
        let (start, end) = slice.split_at_mut(index1);
        Some((&mut end[0], &mut start[index2]))
    }
}

pub fn get_ordering<T>(first: &T, second: &T, rank_order: &[T]) -> Option<Ordering>
where
    T: PartialEq,
{
    // find the index/value for self
    let self_index = rank_order.iter().position(|r| r == first)?;

    // find the index/value for other
    let other_index = rank_order.iter().position(|r| r == second)?;

    // compare
    let ordering = self_index.cmp(&other_index);
    Some(ordering)
}

pub struct DbgFmtFn<F>(pub F)
where
    F: Fn(&mut Formatter) -> Result;

impl<F> Debug for DbgFmtFn<F>
where
    F: Fn(&mut Formatter) -> Result,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        (self.0)(f)
    }
}
