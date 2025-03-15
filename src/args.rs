use url::Url;

use crate::manga::gen_manga;
use crate::model::MangaSource;
use std::env;
use std::error::Error;

static HELP_MSG: &str = r##"blt-dl
Tải Manga từ một số trang nhanh hơn

Usage:
    blt-dl get <url>
    blt-dl get https://ncode.syosetu.com/n6755gk/
Options:
    -h --help       Show this screen.
    -v --version    Show version.
    get             Download the webnovel as an .epub file"##;

pub struct Config {
    pub command: String,
    pub manga_url: String
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        // Including get-file and get-link
        let command = match args.next() {
            Some(arg) => arg,
            None => return Err("Bạn không nhập lệnh gì cả, tự động thoát đây..."),
        };

        // Depended on "command"
        let manga_url = match args.next() {
            Some(arg) => arg,
            None => String::from("..."),
        };

        Ok(Config {
            command,
            manga_url
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let manga_source = match_url(&config.manga_url)?;
    let manga_metadata = manga_source.fetch_metadata(&config.manga_url)?;
    match config.command.as_str() {
        "get" => {
            gen_manga(manga_source, manga_metadata)?;
        },
        "help" => {
            println!("{}", HELP_MSG);
        },
        _ => {
            println!("{}", HELP_MSG)
        }
    }

    Ok(())
}

pub fn match_url(url: &str) -> Result<MangaSource, Box<dyn Error>> {
    match Url::parse(url)?.host_str().unwrap() {
        "kakuyomu.jp" => {
            Ok(Kakuyomu::gen_info())
        },
        "ncode.syosetu.com" | "syosetu.com" => {
            Ok(Syosetu::gen_info())
        },
        _ => {
            Err("No way to be found".into())
        }
    }
}