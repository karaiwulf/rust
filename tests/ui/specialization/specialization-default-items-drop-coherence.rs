//@ revisions: classic coherence next
//@[next] compile-flags: -Znext-solver
//@[coherence] compile-flags: -Znext-solver=coherence
//@[classic] check-pass
//@[classic] known-bug: #105782

// Should fail. Default items completely drop candidates instead of ambiguity,
// which is unsound during coherence, since coherence requires completeness.

#![feature(specialization)]
#![allow(incomplete_features)]

trait Default {
    type Id;
}

impl<T> Default for T {
    default type Id = T; //[next]~ ERROR type annotations needed
}

trait Overlap {
    type Assoc;
}

impl Overlap for u32 {
    type Assoc = usize;
}

impl Overlap for <u32 as Default>::Id {
    //[coherence]~^ ERROR conflicting implementations of trait `Overlap` for type `u32`
    //[next]~^^ ERROR conflicting implementations of trait `Overlap` for type `u32`
    type Assoc = Box<usize>;
}

fn main() {}
