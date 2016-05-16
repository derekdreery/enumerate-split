
/// Don't use this struct directly, use enumerate_split
pub struct EnumerateSplit<Iter, T>
where Iter: Iterator<Item = T>, T: PartialEq {
    iter: Iter,
    split_value: T,
    block_number: usize,
    block_position: usize
}

impl<Iter, T> EnumerateSplit<Iter, T>
where Iter: Iterator<Item = T>, T: PartialEq {

    #[inline]
    fn nudge_block(&mut self) {
        self.block_number += 1;
        self.block_position = 0;
    }

    #[inline]
    fn nudge_position(&mut self) {
        self.block_position += 1;
    }

    #[inline]
    fn get_position(&self) -> (usize, usize) {
        (self.block_number, self.block_position)
    }
}

/// Similar to enumerate but gives (usize, usize) where first
/// is block number and second is block position (indexed by 0)
///
/// Can split on any possible T but useful to use `T=char`, `split_value='\n'`
///
///
/// # Example
/// ```
/// use enumerate_split::enumerate_split;
///
/// let mut input = enumerate_split("Some \n\nstring with a newline".chars(), '\n');
/// assert_eq!(input.next(), Some(('S', (0, 0))));
/// assert_eq!(input.next(), Some(('o', (0, 1))));
/// assert_eq!(input.next(), Some(('m', (0, 2))));
/// assert_eq!(input.next(), Some(('e', (0, 3))));
/// assert_eq!(input.next(), Some((' ', (0, 4))));
/// assert_eq!(input.next(), Some(('\n', (0, 5))));
/// assert_eq!(input.next(), Some(('\n', (1, 0))));
/// assert_eq!(input.next(), Some(('s', (2, 0))));
/// ```
pub fn enumerate_split<Iter, T>(iter: Iter, split_value: T) -> EnumerateSplit<Iter, T>
where Iter: Iterator<Item = T>, T: PartialEq {
    EnumerateSplit {
        iter: iter,
        split_value: split_value,
        block_number: 0,
        block_position: 0
    }
}

impl<Iter, T> Iterator for EnumerateSplit<Iter, T>
where Iter: Iterator<Item = T>, T: PartialEq {
    type Item = (T, (usize, usize));
    fn next(&mut self) -> Option<(T, (usize, usize))> {
        match self.iter.next() {
            Some(t) => {
                let split = t == self.split_value;
                let res = Some((t, self.get_position()));
                if split {
                    self.nudge_block();
                } else {
                    self.nudge_position();
                }
                res
            }
            None => None
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use std::io::prelude::*;

    #[test]
    fn enumerate_split() {
        let mut input = super::enumerate_split("Some \n\nstring with a newline".chars(), '\n');
        assert_eq!(input.next(), Some(('S', (0, 0))));
        assert_eq!(input.next(), Some(('o', (0, 1))));
        assert_eq!(input.next(), Some(('m', (0, 2))));
        assert_eq!(input.next(), Some(('e', (0, 3))));
        assert_eq!(input.next(), Some((' ', (0, 4))));
        assert_eq!(input.next(), Some(('\n', (0, 5))));
        assert_eq!(input.next(), Some(('\n', (1, 0))));
        assert_eq!(input.next(), Some(('s', (2, 0))));
    }
}

