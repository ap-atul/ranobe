use reqwest;

pub fn get_body_from_url(url: &str) -> String {
    let req_error_msg = format!("Failed to request resource :: {}", url);
    let response_parse_error = format!("Failed to parse the response of :: {}", url);

    reqwest::blocking::get(url)
        .expect(&req_error_msg)
        .text()
        .expect(&response_parse_error)
}
