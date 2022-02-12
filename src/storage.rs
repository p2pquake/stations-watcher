use super::retreiver;
use std::{fs, io::Write};

// Strategy パターンっぽくしたかったが思ったようにはなっていない。
// （ Rust にデザインパターンをそのまま当てようとするのが間違っている感）
pub trait Storage {
    fn load(&self) -> Vec<retreiver::SeismicIntensityStation>;
    fn save(&self, stations: Vec<retreiver::SeismicIntensityStation>);
}

pub struct FileStorage;
impl Storage for FileStorage {
    fn load(&self) -> Vec<super::retreiver::SeismicIntensityStation> {
        // FIXME: 初期実行（ファイルが存在しない）を想定する
        let stations_str = fs::read_to_string("stations.json").unwrap();
        let stations = serde_json::from_str(stations_str.as_str()).unwrap();
        stations
    }

    fn save(&self, stations: Vec<super::retreiver::SeismicIntensityStation>) {
        let stations_str = serde_json::to_string(&stations).unwrap();

        let mut file = fs::File::create("stations.json").unwrap();
        file.write_all(stations_str.as_bytes()).unwrap();
    }
}
