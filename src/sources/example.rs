use crate::interface::Source;
use crate::model::*;

pub struct Example {}

impl Source for Example {
    fn metadata(&self) -> DataSource {
        DataSource {
            id: 0,
            name: "".to_string(),
            slug: "".to_string(),
            version: 0,
            base_url: "".to_string(),
            dev_name: "".to_string()
        }
    }

    fn home(&self) -> Vec<Novel> {
        Vec::new()
    }
}
