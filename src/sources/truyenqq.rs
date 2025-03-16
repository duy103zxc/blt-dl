use crate::model::MangaSource;
pub struct TruyenQQ {}

impl TruyenQQ {
    pub fn gen_info() -> MangaSource {
        MangaSource::new(
            // BaseURL
            String::from("https://truyenqqto.com"),
            // manga_name
            String::from(".book_other > h1:nth-child(1)"), 
            String::from(".li01 > a:nth-child(1)"), 
            String::from(".chapter_content"), 
            String::from("div.d-flex:nth-child(4) > a:nth-child(2)")
        )
    }
}