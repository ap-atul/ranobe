use crate::interface::Source;
use crate::model::*;

pub struct Example {}

impl Source for Example {
    fn home(&self) -> Vec<Novel> {
        Vec::new()
    }
}
