use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub enum Status {
    Unknown,
    Ongoing,
    Complete,
    Discontinued,
}

#[derive(Debug)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

pub struct DataSource {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub version: u64,
    pub base_url: String,
    pub dev_name: String,
}

#[derive(Debug)]
pub struct Novel {
    // id of the source
    pub source_id: u64,
    // id of the novel
    pub id: u64,
    pub name: String,
    pub cover: String,
    pub status: Status,
    pub alternate_names: Vec<String>,
    pub authors: Vec<String>,
    pub genres: Vec<Genre>,
    pub url: String,
    pub rating: f32,
    pub year: u16,
    pub chapter_count: u64,
}

pub struct Chapter {
    pub id: u64,
    pub novel_id: u64,
    pub url: String,
    pub name: String,
    pub content: String,
}

impl FromStr for Genre {
    type Err = String;
    fn from_str(s: &str) -> Result<Genre, Self::Err> {
        Ok(Genre {
            id: 0,
            name: s.to_string(),
        })
    }
}
