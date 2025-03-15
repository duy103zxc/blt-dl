use crate::model::MangaSource;

pub struct Syosetu {}

impl Syosetu {
    pub fn gen_info() -> MangaSource {
        MangaSource::new(
            // BaseURL
            String::from("https://ncode.syosetu.com"),
            // manga_name
            String::from(".p-manga__title"), 
            // manga_author
            String::from(".p-manga__author > a:nth-child(1)"), 
            String::from("ja"), 
            String::from("div.p-eplist__sublist:nth-child(1) > a:nth-child(1)"), 
            String::from(".p-manga__title"), 
            String::from(".js-novel-text"), 
            String::from("a.c-pager__item--next") 
        )
    }
}