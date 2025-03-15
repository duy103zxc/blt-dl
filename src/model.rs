use std::error::Error;

use scraper::{Html, Selector};

use crate::utils;


// Save selector
pub struct MangaSource {
    base_url: String,
    manga_name: String,
    first_chapter: String,
    chap_content: String,
    img: String,
    next_chapter: String
}

pub struct MangaMetadata {
    pub manga_name: String,
    pub first_chapter_url: String
}

pub struct Chapter {
	pub content: Vec<String>,
	pub next_chapter_url: String,
    pub any_chapter_left: bool
}

impl MangaSource {
    pub fn new(
        base_url: String,
        manga_name: String, 
        first_chapter: String, 
        chap_content: String, 
        img: String,
        next_chapter: String 
    ) -> Self {
        Self { 
            base_url,
            manga_name, 
            first_chapter, 
            chap_content, 
            img,
            next_chapter 
        }
    }

    pub fn fetch_metadata(&self, manga_url: &str) -> Result<MangaMetadata, Box<dyn Error>> {
        let body = utils::fetch_from_internet(&manga_url)?.body_mut().read_to_string()?;
        let document = Html::parse_document(&body);
        
        let title_selector = Selector::parse(&self.manga_name).unwrap();
        let first_chap_selector = Selector::parse(&self.first_chapter).unwrap();
        
        let manga_name: String = document.select(&title_selector).next().unwrap().text().collect();
        let first_chapter_url: String = document.select(&first_chap_selector).next().unwrap().value().attr("href").unwrap().to_string();
        
        Ok(MangaMetadata {
            manga_name,
            first_chapter_url: format!("{}{}", &self.base_url, first_chapter_url)
        })
    }

    pub fn download_current_chapter(&self, current_url: &str) -> Result<Chapter, Box<dyn Error>> {
        let body = utils::fetch_from_internet(&current_url)?.body_mut().read_to_string()?;
        let document = Html::parse_document(&body);
        let mut chap_lines: Vec<String> = Vec::new();  
        let mut next_chapter_url = String::from("");
        let mut any_chapter_left = true;

        let content_selector = Selector::parse(&self.chap_content).unwrap();
        let next_chap_selector = Selector::parse(&self.next_chapter).unwrap();
        let each_line = Selector::parse("p").unwrap();  
        
        match document.select(&next_chap_selector).next() {
            Some(chapter_url) => {
                next_chapter_url = chapter_url.value().attr("href").unwrap().to_string();
            },
            None => {
                any_chapter_left = false;
            },
        }
        
        ;  

        let chap_content = document.select(&content_selector).next().unwrap();
 
        for chap_line in chap_content.select(&each_line) {
            chap_lines.push(chap_line.text().collect::<String>());
        }

        Ok(Chapter { 
            content: chap_lines, 
            next_chapter_url: format!("{}{}", &self.base_url, next_chapter_url),
            any_chapter_left
        })
    }
}