pub trait Power<P = Self>{
    fn add(&mut self, p: P);

    fn sub(&mut self, p: P);

    fn measure(&self) -> usize;
}


/// Usually use `float` as Power
impl Power for f32{
    fn add(&mut self, p: Self) {
        *self += p;
    }

    fn sub(&mut self, p: Self) {
        *self -= p;
    }

    fn measure(&self) -> usize {
        *self as usize
    }
}