extern crate blis_src;
use peroxide::fuga::*;

fn main() {
    let a = rand(2, 5);
    let b = rand(5, 2);

    let c = &a * &b;

    a.print();
    b.print();
    c.print();
}
