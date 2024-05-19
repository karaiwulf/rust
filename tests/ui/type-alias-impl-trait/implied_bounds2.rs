//@ check-pass

#![feature(type_alias_impl_trait)]

type Ty<'a, A> = impl Sized + 'a;
fn defining<'a, A>() -> Ty<'a, A> {}
fn assert_static<T: 'static>() {}
fn test<'a, A>() where Ty<'a, A>: 'static, { assert_static::<Ty<'a, A>>() }

fn main() {}
