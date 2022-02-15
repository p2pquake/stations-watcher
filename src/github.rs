use std::{
    collections::HashMap,
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub async fn create_issue(
    org: String,
    repo: String,
    app_id: String,
    pem_bucket: String,
    pem_key: String,
    installation_id: String,
    diff: String,
    presigned_url: String,
) {
    let mut map = HashMap::new();
    map.insert("title", "Update seismic intensity stations");
    let body = format!("The list of seismic intensity stations has been updated.\nSee: https://www.data.jma.go.jp/svd/eqev/data/intens-st/stations.json\nCSV: [Stations.csv]({}) (valid for 7 days)\n\n```diff\n{}\n```\n\nCreated by stations_watcher (https://github.com/p2pquake/stations_watcher)", presigned_url, diff);
    map.insert("body", &body);

    let pem = get_app_pem_from_s3(pem_bucket, pem_key).await;
    let jwt = generate_github_app_jwt(pem, app_id).await;
    let access_token = get_access_token(jwt, installation_id).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!(
            "https://api.github.com/repos/{}/{}/issues",
            org, repo
        ))
        .header("Authorization", format!("token {}", access_token))
        .header("User-agent", "stations_watcher")
        .json(&map)
        .send()
        .await
        .unwrap();

    println!("{:?}", resp.text().await);
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iat: usize,
    exp: usize,
    iss: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccessTokensResponse {
    token: String,
}

async fn get_access_token(jwt: String, installation_id: String) -> String {
    // Note. 本来は Installations で一覧取得するステップをはさむ

    // ACCESS_TOKEN を取得
    let client = reqwest::Client::new();
    let resp = client
        .post(format!(
            "https://api.github.com/app/installations/{}/access_tokens",
            installation_id
        ))
        .header("Authorization", format!("Bearer {}", jwt))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-agent", "stations_watcher")
        .send()
        .await;

    match resp {
        Ok(result) => {
            let response_struct = result.json::<AccessTokensResponse>().await.unwrap();
            return response_struct.token;
        }
        Err(error) => {
            println!("access token error!");
            println!("{:?}", error);
            panic!()
        }
    }
}

async fn generate_github_app_jwt(private_key: Vec<u8>, app_id: String) -> String {
    let unixtime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let my_claims = Claims {
        exp: usize::try_from(unixtime + 10 * 60).unwrap(),
        iat: usize::try_from(unixtime - 60).unwrap(),
        iss: app_id,
    };

    let jwt = encode(
        &Header::new(Algorithm::RS256),
        &my_claims,
        &EncodingKey::from_rsa_pem(&private_key).unwrap(),
    )
    .unwrap();

    jwt
}

async fn get_app_pem_from_s3(bucket: String, key: String) -> Vec<u8> {
    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let resp = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .unwrap();

    resp.body
        .collect()
        .await
        .unwrap()
        .into_bytes()
        .as_ref()
        .to_vec()
}
