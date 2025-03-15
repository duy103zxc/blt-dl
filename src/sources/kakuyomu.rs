use crate::model::MangaSource;
pub struct Kakuyomu {}

impl Kakuyomu {
    pub fn gen_info() -> MangaSource {
        MangaSource::new(
            // BaseURL
            String::from("https://kakuyomu.jp"),
            // manga_name
            String::from("h1.Heading_heading__lQ85n"), 
            // manga_author
            String::from(".Gap_size-3s__fjxCP > div:nth-child(2) > div:nth-child(1) > div:nth-child(1) > a:nth-child(1)"), 
            String::from("ja"), 
            String::from(".NewBox_padding-pb-m__8mtGc > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > a:nth-child(1)"), 
            String::from(".widget-episodeTitle"), 
            String::from(".widget-episode-inner"), 
            String::from("#contentMain-readNextEpisode") 
        )
    }
}