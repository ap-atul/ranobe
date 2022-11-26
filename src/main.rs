mod interface;
mod model;
mod sources;

use sources::example::Example;
use crate::interface::Source;

fn main() {
    let source = Example {};

    println!("Selected `Example` source for look up");
    for novel in source.home() {
        println!("{}", novel.name);
    };
}
