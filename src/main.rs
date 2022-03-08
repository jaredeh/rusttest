mod p;
mod tf;
mod c;
mod trans;

use crate::c::jrconftest;
use crate::p::ptest;
use crate::trans::transtest;

fn main() {
    let cfg = jrconftest();
    ptest();
    println!("1Hello, world!");
    transtest();
    println!("2Hello, world!");
}