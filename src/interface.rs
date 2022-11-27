use crate::model::{DataSource, Novel};

pub trait Source {
    fn metadata(&self) -> DataSource;

    // initial novel list
    fn home(&self) -> Vec<Novel>;
}
