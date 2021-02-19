use std::error::Error;

use select::{
    document::Document,
    predicate::{Attr, Name},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let a20_plus_url = "https://zenius-i-vanisher.com/v5.2/viewsimfilecategory.php?categoryid=1293";
    let res_body = client.get(a20_plus_url).send().await?.text().await?;
    let document = Document::from(res_body.as_str());

    // At time of writing, the parts of the song html we care about looked like so
    // <tr>
    //   <td>
    //     ... some unrelated spans ...
    //     <strong>
    //       <a id="sim12345" href="viewsimfile.php?simfileid=12345" title="song name / artist">song name</a>
    //     </strong>
    //   </td>
    //   <td>
    //     <span some attributes>2.5 months ago</span>
    //   </td>
    //   ... many other unimportant td ...
    // </tr>
    for potential_song_node in document.find(Name("tr")) {
        let (song_name, song_url, last_updated_txt) = match extract_song_info(potential_song_node) {
            Some(x) => x,
            None => continue,
        };

        println!(
            "song_name: {}, {} -> {}",
            song_name, last_updated_txt, song_url
        );
    }
    Ok(())
}

fn extract_song_info(potential_song_node: select::node::Node) -> Option<(String, &str, String)> {
    let mut td_iter = potential_song_node.find(Name("td"));

    let song_td = td_iter.next()?;
    let last_updated_td = td_iter.next()?;

    let song_node = song_td.find(Name("strong")).next()?;
    let song_url_node = song_td.find(Attr("href", ())).next()?;

    let song_name = song_node.text();
    // we already know it has an href
    // TODO check it is in the form of viewsimfile.php?simfileid=12345
    let song_url = song_url_node.attr("href").unwrap();

    let last_updated = last_updated_td.find(Name("span")).next()?;
    let last_updated_txt = last_updated.text();

    // TODO parse last_updated_txt to an approximate datetime type

    // TODO make some kind of SongInfo struct to store these in
    Some((song_name, song_url, last_updated_txt))
}
