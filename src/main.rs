extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use similar::{ChangeTag, TextDiff};

mod retreiver;
mod storage;

#[tokio::main]
async fn main() {
    let old_stations = storage::load();
    let stations = retreiver::retreive_and_parse().await;

    let old_stations_str = old_stations.iter().map(|station| station.to_string()).collect::<Vec<_>>().join("\n");
    let stations_str = stations.iter().map(|station| station.to_string()).collect::<Vec<_>>().join("\n");

    let diff = TextDiff::from_lines(&old_stations_str, &stations_str);

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        print!("{}{}", sign, change);
    }


    storage::save(stations);
}
