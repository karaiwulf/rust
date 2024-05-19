//@ revisions: current next
//@[next] compile-flags: -Znext-solver

// cc #119820

trait Trait<'a, 'b> {}

trait OtherTrait<'b> {}
impl<'a, 'b, T: OtherTrait<'b>> Trait<'a, 'b> for T {}

fn impl_hr<'b, T: for<'a> Trait<'a, 'b>>() {}

fn not_hr<'a, T: for<'b> Trait<'a, 'b> + OtherTrait<'static>>() {
    impl_hr::<T>();
    //[next]~^ ERROR the trait bound `for<'a> T: Trait<'a, '_>` is not satisfied
    //[current]~^^ERROR lifetime may not live long enough
    //[current]~| ERROR implementation of `Trait` is not general enough
}

fn main() {}
