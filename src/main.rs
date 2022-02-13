extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use crate::storage::Storage;
use similar::TextDiff;

mod retreiver;
mod storage;

#[tokio::main]
async fn main() {
    let s3_storage =
        storage::S3Storage::new("p2pquake-seismic-intensity-stations".to_string()).await;

    let old_stations = s3_storage.load().await;
    let stations = retreiver::retreive_and_parse().await;

    let old_stations_str = old_stations
        .iter()
        .map(|station| station.to_string())
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    let stations_str = stations
        .iter()
        .map(|station| station.to_string())
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";

    let diff = TextDiff::from_lines(&old_stations_str, &stations_str);

    if diff.ratio() == 1.0 {
        println!("no diff.");
        std::process::exit(0);
    }

    // 変更があった場合
    s3_storage.save(&stations).await;

    println!("{}", diff.unified_diff().context_radius(1));

    let csv = stations
        .iter()
        .map(|station| {
            format!(
                "{},{},{},{},{}",
                station.pref_name(),
                station.name,
                station.lat,
                station.lon,
                station.affi_name()
            )
        })
        .collect::<Vec<_>>()
        .join("\r\n");
    s3_storage.save_csv(csv).await;

    std::process::exit(1);
}
