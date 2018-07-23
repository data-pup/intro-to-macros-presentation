extern crate awoo;
#[macro_use]
extern crate awoo_derive;

use awoo::Awoo;

struct Puppo;

impl Awoo for Puppo {
    fn awoo() {
        println!("I am a Puppo! Awoo!");
    }
}

#[derive(Awoo)]
struct Doggo;

#[derive(Awoo)]
struct Friendo;

fn main() {
    Puppo::awoo();
    Doggo::awoo();
    Friendo::awoo();
}
