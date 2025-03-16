use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use url::Url;
use crate::model::MangaMetadata;
use crate::model::MangaSource;
use crate::utils::fetch_from_internet;
use std::fs::File;

pub fn gen_manga(manga: MangaSource, manga_metadata: MangaMetadata) -> Result<(), Box<dyn Error>> {

    let mut any_chap_left = Some(true);
    let mut current_url = manga_metadata.first_chapter_url;
    let mut current_chap_number = 0;
    
    while let Some(_i) = any_chap_left {
        let current_chapter = manga.download_current_chapter(&current_url)?;
        if current_chapter.any_chapter_left {
            let current_path = format!("./{}/chuong-{}", manga_metadata.manga_name, current_chap_number);
            for image_link in current_chapter.images {
                let mut bytes: Vec<u8> = Vec::new();
                fetch_from_internet(&image_link)?.body_mut().as_reader().read_to_end(&mut bytes)?;
                // Extract the image's filename from URL
                let tmp = Url::parse(&image_link)?;
                let image_filename = Path::new(tmp.path()).file_name().unwrap().to_str().unwrap();
                // Create an empty file to save image data
                let mut data = gen_image(&current_path, image_filename)?;
                data.write_all(&bytes).expect("write failed");
            }
            current_url = current_chapter.next_chapter_url;
            current_chap_number += 1;
        } else {
            any_chap_left = None;
        }
    }

    Ok(())
    
}


fn gen_image(root_folder: &str, file_name: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("{}.{}", root_folder, file_name))
}