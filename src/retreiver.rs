use serde::de;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct SeismicIntensityStation {
    lat: String,
    #[serde(deserialize_with = "deserialize_string_or_float")]
    lon: String,
    name: String,
    pref: String,
    affi: String,
}

impl SeismicIntensityStation {
    pub fn to_string(&self) -> String {
        format!(
            "{},{},{},{},{}",
            self.lat, self.lon, self.name, self.pref, self.affi
        )
    }
}

pub async fn retreive_and_parse() -> Vec<SeismicIntensityStation> {
    let body = reqwest::get("https://www.data.jma.go.jp/svd/eqev/data/intens-st/stations.json")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let items: Vec<SeismicIntensityStation> = serde_json::from_str(&body).unwrap();
    items
}

fn deserialize_string_or_float<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeStringOrFloatVisitor)
}

struct DeserializeStringOrFloatVisitor;

impl<'de> de::Visitor<'de> for DeserializeStringOrFloatVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an float or a string")
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }
}
