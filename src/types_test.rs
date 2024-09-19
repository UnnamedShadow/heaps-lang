trait add<T> {
    type Output;
    fn add(&self, rhs: T) -> Self::Output;
}

impl add<String> for String {
    type Output = String;
    fn add(&self, rhs: String) -> Self::Output {
        format!("{}{}", self, rhs).to_string()
    }
}

///
/// heaps_lang::heaps_sync! {
///     add1(a, b) < {
///         add(a, b)
///     }
///     add2(a) < {
///         add1(a, "world")
///     }
///     add3() < {
///         add2("hello ")
///     }
/// }
///

trait add1Trait<b> {
    type Output;
    fn add1(&self, b: b) -> Self::Output;
}
impl<a, b> add1Trait<b> for a
where
    Self: add<b>,
    Self: Clone,
{
    type Output = <a as add<b>>::Output;
    fn add1(&self, b: b) -> Self::Output {
        let a = self.clone();
        a.add(b)
    }
}

trait add2Trait {
    type Output;
    fn add2(&self) -> Self::Output;
}
impl<a> add2Trait for a
where
    Self: add1Trait<String>,
    Self: Clone,
{
    type Output = <a as add1Trait<String>>::Output;
    fn add2(&self) -> Self::Output {
        let a = self.clone();
        a.add1("world".to_string())
    }
}

type add3 = <String as add2Trait>::Output;
fn add3() -> add3 {
    "hello ".to_string().add2()
}

trait add4Trait<b, c> {
    type Output;
    fn add4(&self, b: b, c: c) -> Self::Output;
}
impl<a, b, c> add4Trait<b, c> for a
where
    b: add<c>,
    Self: add<<b as add<c>>::Output>,
    Self: Clone,
{
    type Output = <a as add<<b as add<c>>::Output>>::Output;
    fn add4(&self, b: b, c: c) -> Self::Output {
        let a = self.clone();
        let d = b.add(c);
        a.add(d)
    }
}
