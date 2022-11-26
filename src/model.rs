use std::str::FromStr;

pub enum Status {
    Ongoing,
    Complete,
    Discontinued,
}

pub struct Genre {
    id: u64,
    name: String,
}

pub struct Novel {
    // id of the source
    source_id: u64,
    // id of the novel
    id: u64,
    pub(crate) name: String,
    cover: String,
    status: Status,
    alternate_names: Vec<String>,
    authors: Vec<String>,
    genres: Vec<Genre>,
    url: String,
    rating: u8,
    year: u16,
    chapter_count: u64,
}

pub struct Chapter {
    id: u64,
    novel_id: u64,
    url: String,
    name: String,
    content: String,
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

impl Default for Novel {
    fn default() -> Self {
        Novel {
            source_id: 0,
            // id of the novel
            id: 0,
            name: String::from("NA"),
            cover: String::from(""),
            status: Status::Complete,
            alternate_names: Vec::new(),
            authors: Vec::new(),
            genres: Vec::new(),
            url: String::from(""),
            rating: 0,
            year: 0,
            chapter_count: 0,
        }
    }
}
