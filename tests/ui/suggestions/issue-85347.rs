use std::ops::Deref;
trait Foo {
    type Bar<'a>: Deref<Target = <Self>::Bar<Target = Self>>;
    //~^ ERROR associated type takes 1 lifetime argument but 0 lifetime arguments were supplied
    //~| HELP add missing
    //~| ERROR associated type bindings are not allowed here
    //~| HELP consider removing this type binding
    //~| ERROR associated type takes 1 lifetime argument but 0 lifetime arguments were supplied
    //~| HELP add missing
    //~| ERROR associated type bindings are not allowed here
    //~| HELP consider removing this type binding
}

fn main() {}
