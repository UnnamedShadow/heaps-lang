use heaps_std::*;

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
//     fn fa(self, b: b) -> Self::Output;
// }
// impl<a, b> faTrait<b> for a
// where
//     a: addTrait<b>,
// {
//     type Output = <a as addTrait<b>>::Output;
//     fn fa(self, b: b) -> Self::Output {
//         let a = self;
//         a.add(b)
//     }
// }
// trait fbTrait {
//     type Output;
//     fn fb(self) -> Self::Output;
// }
// impl<a> fbTrait for a
// where
//     a: faTrait<heaps_std::str::Str>,
// {
//     type Output = <a as faTrait<heaps_std::str::Str>>::Output;
//     fn fb(self) -> Self::Output {
//         let a = self;
//         a.fa(heaps_std::str::Str("world".to_string()))
//     }
// }
// type fcOutput = <heaps_std::str::Str as fbTrait>::Output;
// fn fc() -> fcOutput {
//     heaps_std::str::Str("hello ".to_string()).fb()
// }
// type mainOutput = ();
// fn main() -> mainOutput {
//     fc().print();
// }
