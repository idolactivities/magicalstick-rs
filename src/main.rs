#[macro_use]
extern crate clap;
extern crate reqwest;
extern crate scraper;
extern crate url;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate try_print;

use scraper::{Html, Selector};
use url::Url;
use url::percent_encoding::percent_decode;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        UrlParseError(url::ParseError);
        Utf8ParseError(std::string::FromUtf8Error);
    }
}

const HOST: &'static str = "http://sanabutton.ojaru.jp";
const EXT: &'static str = "mp3";

fn run() -> Result<()> {
    let matches = clap_app!(magicalstick =>
                            (version: "0.0.2")
                            (about: "Show sana button resource list.")
                            (@arg percents: -u --urlencode "Prints out percent-encoded URLs")
                           ).get_matches();
    let mut urls: Vec<String> = Vec::new();
    let sounds_selector = Selector::parse(r#".sounds"#).unwrap();
    for sound in Html::parse_document(&reqwest::get(HOST)?.text()?).select(&sounds_selector) {
        urls.push(
            Url::parse(
                &format!("{}/{}.{}", HOST, sound.value().attr("data-file").unwrap_or(""), EXT)
            )?
            .into_string()
        );
    }
    if matches.is_present("percents") {
        if let Err(_) = try_println!("{}", urls.join("\n")) {
            std::process::exit(0);
        }
    } else {
        let mut pretty_urls: Vec<String> = Vec::new();
        for url in urls {
            pretty_urls.push(
                String::from_utf8(percent_decode(url.into_bytes().as_slice()).collect())?
            );
        }
        if let Err(_) = try_println!("{}", pretty_urls.join("\n")) {
            std::process::exit(0);
        }
    }
    Ok(())
}

quick_main!(run);
