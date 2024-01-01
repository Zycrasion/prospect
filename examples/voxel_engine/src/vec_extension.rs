pub trait VecPushIndex
{
    type Item;
    fn pushi(&mut self, value : Self::Item) -> usize;
}

impl<T> VecPushIndex for Vec<T>
{
    type Item = T;
    fn pushi(&mut self, value : Self::Item) -> usize{
        self.push(value);
        self.len() - 1usize
    }
}