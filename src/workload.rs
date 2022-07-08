use std::collections::HashMap;
use std::hash::Hash;

pub trait Workload<W = Self>{
    fn add(&mut self, p: W);

    fn sub(&mut self, p: W);

    fn len(&self) -> usize;

    fn from(data:Vec<u8>) -> Self;
}

impl<T> Workload for Vec<T>
where T: From<Vec<u8>> + Eq + PartialEq + Clone
{
    fn add(&mut self, p: Self) {
        for w in p{
            self.push(w.clone());
        }
    }

    fn sub(&mut self, p: Self) {
        for w in p {
            self.retain(|inner|{
                &w != inner
            })
        }
    }

    fn len(&self) -> usize{
        self.len()
    }

    fn from(data: Vec<u8>) -> Self {
        vec!(data.into())
    }

}
