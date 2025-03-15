use std::error::Error;
use std::fs::OpenOptions;
use crate::model::MangaMetadata;
use crate::model::MangaSource;
use crate::utils::fetch_from_internet;
use std::{fs::File, io::copy};

pub fn gen_manga(manga: MangaSource, manga_metadata: MangaMetadata) -> Result<(), Box<dyn Error>> {

    let mut any_chap_left = Some(true);
    let mut current_url = manga_metadata.first_chapter_url;
    let mut current_chap_number = 0;

    while let Some(_i) = any_chap_left {
        let current_chapter = manga.download_current_chapter(&current_url)?;
        if current_chapter.any_chapter_left {
            // Generate each images here by fetching bytes from the URL and then save them as images.
            let current_img = fetch_from_internet(&current_url)?.body_mut();
            let img_name = check_img_ext(&current_url);
            // Create a new file to write the downloaded image to
            let data = gen_image(img_name.0, img_name.1)?;
            // Copy the contents of the response to the file
            copy(&mut current_img, &mut data)?;
            current_url = current_chapter.next_chapter_url;
            current_chap_number += 1;
        } else {
            any_chap_left = None;
        }
    }

    Ok(())
    
}


fn gen_image(file_name: &str, img_ext: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("{}.{}", file_name, img_ext))
}

fn check_img_ext(link: &str) -> (&str, &str) {
    ("", "")
}