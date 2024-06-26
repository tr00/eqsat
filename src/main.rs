// mod intersect;
use std::collections::HashSet;
mod intersect;

use crate::intersect::intersect;

fn main() {
    let mut a = HashSet::new();
    let mut b = HashSet::new();

    a.insert(4);
    a.insert(5);
    a.insert(6);

    b.insert(1);
    b.insert(5);
    b.insert(9);

    let mut v = vec![a, b];

    let c = intersect(&mut v);

    for val in c {
        println!("{}", val);
    }
}
