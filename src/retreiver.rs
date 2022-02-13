use serde::de;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct SeismicIntensityStation {
    pub lat: String,
    #[serde(deserialize_with = "deserialize_string_or_float")]
    pub lon: String,
    pub name: String,
    pub pref: String,
    pub affi: String,
}

impl SeismicIntensityStation {
    pub fn to_string(&self) -> String {
        format!(
            "{},{},{},{},{}",
            self.lat, self.lon, self.name, self.pref, self.affi
        )
    }

    pub fn pref_name(&self) -> String {
        let str = self.pref.as_str();

        let pref = match str {
            "1" => "北海道",
            "2" => "青森県",
            "3" => "岩手県",
            "4" => "宮城県",
            "5" => "秋田県",
            "6" => "山形県",
            "7" => "福島県",
            "8" => "茨城県",
            "9" => "栃木県",
            "10" => "群馬県",
            "11" => "埼玉県",
            "12" => "千葉県",
            "13" => "東京都",
            "14" => "神奈川県",
            "15" => "新潟県",
            "16" => "富山県",
            "17" => "石川県",
            "18" => "福井県",
            "19" => "山梨県",
            "20" => "長野県",
            "21" => "岐阜県",
            "22" => "静岡県",
            "23" => "愛知県",
            "24" => "三重県",
            "25" => "滋賀県",
            "26" => "京都府",
            "27" => "大阪府",
            "28" => "兵庫県",
            "29" => "奈良県",
            "30" => "和歌山県",
            "31" => "鳥取県",
            "32" => "島根県",
            "33" => "岡山県",
            "34" => "広島県",
            "35" => "山口県",
            "36" => "徳島県",
            "37" => "香川県",
            "38" => "愛媛県",
            "39" => "高知県",
            "40" => "福岡県",
            "41" => "佐賀県",
            "42" => "長崎県",
            "43" => "熊本県",
            "44" => "大分県",
            "45" => "宮崎県",
            "46" => "鹿児島県",
            "47" => "沖縄県",
            _ => "不明",
        };

        pref.to_string()
    }

    pub fn affi_name(&self) -> String {
        if self.affi == "0".to_string() {
            "気象庁".to_string()
        } else if self.affi == "1".to_string() {
            "地方公共団体".to_string()
        } else {
            "防災科研".to_string()
        }
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
