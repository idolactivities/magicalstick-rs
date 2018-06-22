extern crate reqwest;
extern crate scraper;
extern crate url;
#[macro_use]
extern crate error_chain;

use scraper::{Html, Selector};
use url::Url;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        UrlParseError(url::ParseError);
    }
}

const HOST: &'static str = "http://sanabutton.ojaru.jp";
const EXT: &'static str = "mp3";

fn run() -> Result<()> {
    let sounds_selector = Selector::parse(r#".sounds"#).unwrap();
    for sound in Html::parse_document(&reqwest::get(HOST)?.text()?).select(&sounds_selector) {
        let url = Url::parse(&format!("{}/{}.{}", HOST, sound.value().attr("data-file").unwrap_or(""), EXT))?;
        println!("{}", url.as_str());
    }
    Ok(())
}

quick_main!(run);
