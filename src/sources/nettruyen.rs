use crate::model::MangaSource;

pub struct Nettruyen {}

impl Nettruyen {
    pub fn gen_info() -> MangaSource {
        MangaSource::new(
            // BaseURL
            String::from("https://nettruyenx.com"),
            String::from(".title-detail"), 
            String::from("a.btn-warning:nth-child(1)"), 
            String::from(".reading-detail"),
            // CSS Selector for images
            String::from("img"),
            String::from("a.btn-danger:nth-child(2)") 
        )
    }
}