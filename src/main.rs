use reqwest;
use scraper::{Html, Selector};

const BASE_URL: &str = "https://books.toscrape.com";

fn get_response() -> String {
    reqwest::blocking::get(BASE_URL)
        .expect("Failed to load the base url")
        .text()
        .unwrap()
}

fn main() {
    let body = get_response();
    let document = Html::parse_document(&body);

    let book_selector = Selector::parse("article.product_pod").unwrap();
    let book_name_selector = Selector::parse("h3 a").unwrap();
    let book_price_selector = Selector::parse(".price_color").unwrap();
    let book_link_selector = Selector::parse("h3 a").unwrap();

    for element in document.select(&book_selector) {
        let name = element.select(&book_name_selector)
            .next()
            .expect("could not select book name")
            .value()
            .attr("title")
            .expect("could not select title of book name");

        let price = element.select(&book_price_selector)
            .next()
            .expect("could not select book price element")
            .text()
            .collect::<String>();

        let link = element.select(&book_link_selector)
            .next()
            .expect("could not select book link element")
            .value()
            .attr("href")
            .expect("could not get the href attr for link element");
    }
}
