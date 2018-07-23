extern crate awoo;

use awoo::Awoo;

struct Puppo;

impl Awoo for Puppo {
    fn awoo() {
        println!("I'm a puppo! Awooooo!");
    }
}

fn main() {
    Puppo::awoo();
}
