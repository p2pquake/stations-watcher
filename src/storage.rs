use std::{fs, io::Write};

pub fn load() -> Vec<super::retreiver::SeismicIntensityStation> {
  let stations_str = fs::read_to_string("stations.json").unwrap();
  let stations = serde_json::from_str(stations_str.as_str()).unwrap();
  stations
}

pub fn save(stations: Vec<super::retreiver::SeismicIntensityStation>) {
  let stations_str = serde_json::to_string(&stations).unwrap();

  let mut file = fs::File::create("stations.json").unwrap();
  file.write_all(stations_str.as_bytes()).unwrap();
}