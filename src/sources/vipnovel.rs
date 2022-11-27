use std::borrow::Borrow;
use std::str::FromStr;

use scraper::{Html, Selector};

use crate::interface::Source;
use crate::model::*;
use crate::utils::file;
use crate::utils::http;
use crate::utils::text;

pub struct VipNovel {}

impl Source for VipNovel {
    fn metadata(&self) -> DataSource {
        DataSource {
            id: 1,
            name: "Vip Novel".to_string(),
            slug: "vip-novel".to_string(),
            version: 0,
            base_url: "https://vipnovel.com/".to_string(),
            dev_name: "ap-atul".to_string(),
        }
    }

    fn home(&self) -> Vec<Novel> {
        let base_url = self.metadata().base_url;
        let body = http::get_body_from_url(&base_url);
        let document = Html::parse_document(&body);

        let list_selector = Selector::parse("div.page-item-detail").unwrap();
        let id_selector = Selector::parse("div.item-thumb").unwrap();
        let link_selector = Selector::parse("a").unwrap();
        let image_selector = Selector::parse("img.img-responsive").unwrap();
        let summary_selector = Selector::parse("div.item-summary").unwrap();
        let title_selector = Selector::parse("div.font-title h3 a").unwrap();
        let rating_selector = Selector::parse("div.rating span.score").unwrap();
        let latest_chap_selector = Selector::parse("div.chapter-item a.btn-link").unwrap();

        let mut result: Vec<Novel> = Vec::new();
        for element in document.select(&list_selector) {
            let basic_details = element.select(&id_selector).next().unwrap();
            let summary = element.select(&summary_selector).next().unwrap();

            let id = u64::from_str(basic_details.value().attr("data-post-id").unwrap()).unwrap_or(0);
            let url = basic_details.select(&link_selector).next().unwrap().value().attr("href").unwrap();
            let cover = basic_details.select(&image_selector).next().unwrap().value().attr("src").unwrap();
            let name = summary.select(&title_selector).next().unwrap().text().collect::<String>();
            let score = f32::from_str(&summary.select(&rating_selector).next().unwrap().text().collect::<String>()).unwrap();
            let latest_chapter_string = summary.select(&latest_chap_selector).next().unwrap().text().collect::<String>();
            let latest_chapter = u64::from_str(text::parse_int_from_str(&latest_chapter_string)).unwrap_or(0);

            result.push(Novel {
                source_id: self.metadata().id,
                id,
                name: text::clean(&name),
                cover: cover.to_string(),
                status: Status::Unknown,
                alternate_names: vec![],
                authors: vec![],
                genres: vec![],
                url: url.to_string(),
                rating: score,
                year: 0,
                chapter_count: latest_chapter,
            });
        }

        result
    }
}
