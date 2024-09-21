trait addTrait<T> {
    type Output;
    fn add(&self, rhs: T) -> Self::Output;
}
impl addTrait<String> for String {
    type Output = String;
    fn add(&self, rhs: String) -> Self::Output {
        format!("{}{}", self, rhs).to_string()
    }
}

trait printTrait {
    type Output;
    fn print(&self) -> Self::Output;
}
impl printTrait for String {
    type Output = ();
    fn print(&self) -> Self::Output {
        println!("{}", self);
    }
}

heaps_lang::heaps_sync! {
    fa(a b) = {
        add(a, b)
    }
    fb(a) = {
        fa(a, "world")
    }
    fc() = {
        fb("hello ")
    }
    main() = {
        print(fc());
    }
}
// should expand to:
// trait faTrait<b> {
//     type Output;
//     fn fa(&self, b: b) -> Self::Output;
// }
// impl<a, b> faTrait<b> for a
// where
//     Self: Clone,
//     a: addTrait<b>,
// {
//     type Output = <a as addTrait<b>>::Output;
//     fn fa(&self, b: b) -> Self::Output {
//         let a = self.clone();
//         a.add(b)
//     }
// }

// trait fbTrait {
//     type Output;
//     fn fb(&self) -> Self::Output;
// }
// impl<a> fbTrait for a
// where
//     Self: Clone,
//     a: faTrait<String>,
// {
//     type Output = <a as faTrait<String>>::Output;
//     fn fb(&self) -> Self::Output {
//         let a = self.clone();
//         a.fa("world".to_string())
//     }
// }

// type fcOutput = <String as fbTrait>::Output;
// fn fc() -> fcOutput {
//     "hello ".to_string().fb()
// }

// type mainOutput = ();
// fn main() -> mainOutput {
//     fc().print();
// }
