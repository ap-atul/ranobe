use sources::example::Example;

use crate::interface::Source;

mod interface;
mod model;
mod sources;

fn main() {
    let source = Example {};

    println!("Selected `Example` source for look up");
    for novel in source.home() {
        println!("{}", novel.name);
    };
}
