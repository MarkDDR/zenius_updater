use std::error::Error;

use select::{document::Document, predicate::{Attr, Name}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let a20_plus_url = "https://zenius-i-vanisher.com/v5.2/viewsimfilecategory.php?categoryid=1293";
    let res_body = client.get(a20_plus_url).send().await?.text().await?;
    let document = Document::from(res_body.as_str());

    for potential_song_node in document.find(Name("tr")) {
        let mut td_iter = potential_song_node.find(Name("td"));
        let song_td = match td_iter.next() {
            Some(n) => n,
            None => continue,
        };
        let last_updated_td = match td_iter.next() {
            Some(n) => n,
            None => continue,
        };

        let song_node = match song_td.find(Name("strong")).next() {
            Some(n) => n,
            None => continue,
        };
        let song_url_node = match song_td.find(Attr("href", ())).next() {
            Some(n) => n,
            None => continue,
        };

        let song_name = song_node.text();
        let song_url = song_url_node.attr("href").unwrap();

        let last_updated = match last_updated_td.find(Name("span")).next() {
            Some(n) => n,
            None => continue,
        };

        let last_updated_txt = last_updated.text();

        println!("song_name: {}, {} -> {}", song_name, last_updated_txt, song_url);
    }
    Ok(())
}
