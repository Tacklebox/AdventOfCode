#[derive(Debug)]
pub struct DropNth<I> {
    count: usize,
    iter: I,
    n: usize,
}

impl<I> DropNth<I>
where
    I: Iterator,
{
    pub fn new(n: usize, iter: I) -> Self {
        Self { count: 0, iter, n }
    }
}

impl<I> Iterator for DropNth<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.n {
            self.count += 1;
            let _ = self.iter.next();
        }
        self.count += 1;
        self.iter.next()
    }
}

pub trait IteratorExt: Iterator {
    fn drop_nth(self, n: usize) -> DropNth<Self>
    where
        Self: Sized,
    {
        DropNth::new(n, self)
    }
}

impl<I: Iterator> IteratorExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_drops_the_nth() {
        let no_four = DropNth::new(4, 0..10).collect::<Vec<_>>();
        assert_eq!(no_four, vec![0, 1, 2, 3, 5, 6, 7, 8, 9]);
    }
}
