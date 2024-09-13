use std::ops::Add;

trait add1 {
    type Output;
    fn add1(self, _: i32) -> Self::Output;
}

impl<T> add1 for T
where
    T: Add<i32, Output = T>,
{
    type Output = T;
    fn add1(self, _: i32) -> Self::Output {
        self + 1
    }
}

fn add2<X>(x: X) -> X::Output
where
    X: add1,
{
    x.add1(1)
}
