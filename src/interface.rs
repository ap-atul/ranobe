use crate::model::Novel;

pub trait Source {
    // initial novel list
    fn home(&self) -> Vec<Novel>;
}
