use std::{collections::HashMap, path, time::Duration};

use path::PathBuf;
use select::{
    document::Document,
    predicate::{Attr, Name, Predicate},
};

use anyhow::{Context, Result};
use tokio::fs::read_dir;

// TODO custom error type with thiserror or snafu or something

#[tokio::main]
async fn main() -> Result<()> {
    let a20_path = "/home/mark/.stepmania-5.1/Songs/DDR A20\ PLUS";
    let blah = walk_song_folder(a20_path).await?;

    // let client = reqwest::Client::new();
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    let a20_plus_url = "https://zenius-i-vanisher.com/v5.2/viewsimfilecategory.php?categoryid=1293";
    println!("Sending request to zenius...");
    let res_body = client.get(a20_plus_url).send().await?.text().await?;
    println!("Got Response back from zenius!");
    let document = Document::from(res_body.as_str());

    for potential_song_node in document.find(Name("tr")) {
        let songinfo = match extract_song_info(potential_song_node) {
            Some(x) => x,
            None => continue,
        };

        println!("{:?}", songinfo);
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct SongInfoZenius {
    song_name: String,
    url: String,
    last_updated: String,
}

fn extract_song_info(potential_song_node: select::node::Node) -> Option<SongInfoZenius> {
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
    let url_ending = song_node.attr("href").unwrap();
    let url = format!("https://zenius-i-vanisher.com/v5.2/{}", url_ending);

    let last_updated = td_last_updated.find(Name("span")).next()?;
    let last_updated_txt = last_updated.text();

    // TODO parse last_updated_txt to an approximate datetime type
    // the actual song page has a better datetime we could use there, but web calls
    // to zenius are really slow so we want to avoid it if we can

    Some(SongInfoZenius {
        song_name,
        url,
        last_updated: last_updated_txt,
    })
}

struct SongInfoFS {
    song_name: String,
    last_updated: Duration,
    path: PathBuf,
    components: HashMap<String, Duration>,
}

async fn walk_song_folder(version_folder: &str) -> Result<HashMap<String, SongInfoFS>> {
    // let mut songs = HashMap::new();
    let mut walker = read_dir(version_folder)
        .await
        .context("invalid song folder path")?;
    while let Some(entry) = walker.next_entry().await? {
        let file_type = entry.file_type().await?;
        if !file_type.is_dir() {
            continue;
        }
        println!("{:?}", entry.path());
        continue;
        let mut song_walker = read_dir(entry.path()).await?;
        while let Some(component) = song_walker.next_entry().await? {}
    }
    todo!()
}
