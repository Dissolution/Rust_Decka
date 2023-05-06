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


pub struct Fmt<F>(pub F)
    where F: Fn(&mut Formatter) -> Result;

impl<F> Debug for Fmt<F>
    where F: Fn(&mut Formatter) -> Result
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        (self.0)(f)
    }
}

pub trait LongDisplay {
    fn display(&self, f: &mut Formatter<'_>) -> Result;
}

pub trait ShortDisplay<T: Display = Self>: Display {
    fn short_display(&self, f: &mut Formatter<'_>) -> Result;
}