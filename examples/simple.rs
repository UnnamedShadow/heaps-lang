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
    fa(a b) < {
        add(a, b)
    }
    fb(a) < {
        fa(a, "world")
    }
    fc() < {
        fb("hello ")
    }
    main() < {
        print(fc());
    }
}
