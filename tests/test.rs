//! some dummy tests

extern crate loosen;
use loosen::loose;

pub struct A;
pub struct B;

#[loose]
fn fa(_a: A, _b: B) {}

#[loose]
pub fn fb(_a: A, _b: B) -> () {}

#[loose]
pub unsafe fn fc(_a: A, _b: B) {}

#[loose]
pub const fn fd(_a: A, _b: B) {}

#[loose]
pub const unsafe fn fe(_a: &A, _b: &B) {}

#[loose]
pub const unsafe fn ff<X, Y>(_a: &X, _b: &Y) {}

#[test]
fn works() {
    fa_loose((A, B));
    fb_loose((A, B));
    unsafe {
        fc_loose((A, B));
    }
    fd_loose((A, B));
    unsafe {
        fe_loose((&A, &B));
    }
    unsafe {
        ff_loose::<A, B>((&A, &B));
    }
}

#[test]
fn iter() {
    (0..10)
        .map(|_| (A, B))
        .map(fa_loose)
        .collect::<Vec<()>>()
        .into_iter()
        .collect::<()>()
}
