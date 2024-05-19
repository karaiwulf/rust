// See https://github.com/rust-lang/rust/issues/88442
//@ run-rustfix
//@ edition:2018
//@ check-pass
#![allow(unused)]
#![warn(rust_2021_prelude_collisions)]

trait AnnotatableTryInto {
    fn try_into<T>(self) -> Result<T, Self::Error>
    where Self: std::convert::TryInto<T> {
        std::convert::TryInto::try_into(self)
    }
}

impl<T> AnnotatableTryInto for T where T: From<u8> {}

fn main() -> Result<(), &'static str> {
    let x: u64 = 1;
    x.try_into::<usize>().or(Err("foo"))?.checked_sub(1);
    //~^ WARNING trait method `try_into` will become ambiguous in Rust 2021
    //~| WARNING this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!

    x.try_into::<usize>().or(Err("foo"))?;
    //~^ WARNING trait method `try_into` will become ambiguous in Rust 2021
    //~| WARNING this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!

    Ok(())
}
