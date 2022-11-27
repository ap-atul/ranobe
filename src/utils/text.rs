use regex::Regex;

pub fn clean(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn parse_int_from_str(s: &str) -> &str {
    let re = Regex::new(r"\d+").unwrap();
    re.find(s).unwrap().as_str()
}
