// Check that revisioned `run-rustfix` does not fail with issues related to `.` in crate name.
//@ revisions: foo
//@[foo] run-rustfix
#![deny(unused_variables)]

fn main() {
    let x = 0usize;
    //~^ ERROR unused variable: `x`
}
