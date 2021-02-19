use std::error::Error;

use select::{
    document::Document,
    predicate::{Attr, Name, Predicate},
};

// TODO custom error type with thiserror or snafu or something

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let a20_plus_url = "https://zenius-i-vanisher.com/v5.2/viewsimfilecategory.php?categoryid=1293";
    let res_body = client.get(a20_plus_url).send().await?.text().await?;
    let document = Document::from(res_body.as_str());

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
    // At time of writing (2020-02-19), the parts of the song html we care about looked like so
    // <tr>
    //   <td>
    //     ... some unrelated spans ...
    //     <strong>
    //       <a id="sim12345" href="viewsimfile.php?simfileid=12345" title="song name / artist">song name</a>
    //     </strong>
    //   </td>
    //   <td>
    //     <span style="something">2.5 months ago</span>
    //   </td>
    //   ... many other unimportant td ...
    // </tr>
    let mut td_iter = potential_song_node.find(Name("td"));

    let td_songname = td_iter.next()?;
    let td_last_updated = td_iter.next()?;

    let song_node = td_songname
        .find(
            Name("strong")
                .descendant(Name("a"))
                .and(Attr("id", ()))
                .and(Attr("href", ()))
                .and(Attr("title", ())),
        )
        .next()?;

    let song_name = song_node.text();
    // TODO check it is in the form of viewsimfile.php?simfileid=12345
    let song_url = song_node.attr("href").unwrap();

    let last_updated = td_last_updated.find(Name("span")).next()?;
    let last_updated_txt = last_updated.text();

    // TODO parse last_updated_txt to an approximate datetime type
    // the actual song page has a better datetime we could use there, but web calls
    // to zenius are really slow so we want to avoid it if we can

    // TODO make some kind of SongInfo struct to store these in
    Some((song_name, song_url, last_updated_txt))
}
