#![feature(unsafe_destructor)]

fn main() {
    trait Trait<T> {}
    struct Struct<S>(S);

    #[unsafe_destructor]
    impl<S, T: Trait<S>> Drop for Struct<T> {
        fn drop(&mut self) {}
    }

    Struct(());
}
