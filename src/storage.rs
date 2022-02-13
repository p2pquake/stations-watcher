use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{ByteStream, Client};

use crate::retreiver::retreive_and_parse;

use super::retreiver;
use std::{fs, io::Write};

// Strategy パターンっぽくしたかったが思ったようにはなっていない。
// （ Rust にデザインパターンをそのまま当てようとするのが間違っている感）
#[async_trait]
pub trait Storage {
    async fn load(&self) -> Vec<retreiver::SeismicIntensityStation>;
    async fn save(&self, stations: Vec<retreiver::SeismicIntensityStation>);
}

pub struct FileStorage;
#[async_trait]
impl Storage for FileStorage {
    async fn load(&self) -> Vec<super::retreiver::SeismicIntensityStation> {
        // FIXME: 初期実行（ファイルが存在しない）を想定する
        let stations_str = fs::read_to_string("stations.json").unwrap();
        let stations = serde_json::from_str(stations_str.as_str()).unwrap();
        stations
    }

    async fn save(&self, stations: Vec<super::retreiver::SeismicIntensityStation>) {
        let stations_str = serde_json::to_string(&stations).unwrap();

        let mut file = fs::File::create("stations.json").unwrap();
        file.write_all(stations_str.as_bytes()).unwrap();
    }
}

pub struct S3Storage {
    bucket: String,
    key: String,
    client: Client,
}

impl S3Storage {
    pub async fn new(bucket: String) -> Self {
        let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);

        Self {
            bucket: bucket,
            key: "stations.json".to_string(),
            client: client,
        }
    }
}

#[async_trait]
impl Storage for S3Storage {
    async fn load(&self) -> Vec<retreiver::SeismicIntensityStation> {
        let resp = self
            .client
            .get_object()
            .bucket(self.bucket.to_string())
            .key(self.key.to_string())
            .send()
            .await;

        if resp.is_err() {
            // FIXME: エラー処理 (ファイルが存在しない場合の処理は必要)
            return vec![];
        }

        let bytes = resp.unwrap().body.collect().await.unwrap().into_bytes();
        let text = String::from_utf8_lossy(bytes.as_ref());

        serde_json::from_str(text.as_ref()).unwrap()
    }

    async fn save(&self, stations: Vec<retreiver::SeismicIntensityStation>) {
        let stations_str = serde_json::to_string(&stations).unwrap();
        let bytestream = ByteStream::from(stations_str.as_bytes().to_vec());

        let resp = self
            .client
            .put_object()
            .bucket(self.bucket.to_string())
            .key(self.key.to_string())
            .body(bytestream)
            .send()
            .await
            .unwrap();
    }
}
