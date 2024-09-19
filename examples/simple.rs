heaps_lang::heaps_sync! {
    f1(a, b) < {
        add(a, b)
    }
    f2(a) < {
        f1(a, "world")
    }
    f3() < {
        f2("hello ")
    }
    main() < {
        print(f3());
    }
}
