use sources::example::Example;

use crate::interface::Source;

mod interface;
mod model;
mod sources;
mod utils;

fn main() {
    let vip_novel = sources::vipnovel::VipNovel {};
    for novel in vip_novel.home() {
        println!("{:?}", &novel);
    }
}
