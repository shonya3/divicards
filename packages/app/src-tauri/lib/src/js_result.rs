use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum JSResult<T> {
    #[serde(rename = "ok")]
    Ok { data: T },
    #[serde(rename = "err")]
    Err { error: String },
}

impl<T, E> From<Result<T, E>> for JSResult<T>
where
    E: Display,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(data) => JSResult::Ok { data },
            Err(err) => JSResult::Err {
                error: err.to_string(),
            },
        }
    }
}

mod test {
    #![allow(unused)]
    use std::{
        fs::{read_to_string, File},
        io::Write,
    };

    use crate::js_result::JSResult;

    use divi::{sample::DivinationCardsSample, Prices};

    #[tokio::test]
    async fn jsres() {
        let csv = read_to_string("./divi/example-2.csv").unwrap();
        let sample = DivinationCardsSample::create(
            divi::SampleData::CsvString(csv),
            Prices::fetch(&divi::TradeLeague::Crucible).await.unwrap(),
        );

        let js_result = JSResult::from(sample);
        let json = serde_json::to_string(&js_result).unwrap();

        let mut file = File::create("js_result.json").unwrap();
        file.write_all(&json.as_bytes()).unwrap();
    }

    #[tokio::test]
    async fn jsres_err() {
        let csv = String::from("not a valid csv lol");
        let sample = DivinationCardsSample::create(
            divi::SampleData::CsvString(csv),
            Prices::fetch(&divi::TradeLeague::Crucible).await.unwrap(),
        );

        let js_result = JSResult::from(sample);
        let json = serde_json::to_string(&js_result).unwrap();

        let mut file = File::create("js_result_err.json").unwrap();
        file.write_all(&json.as_bytes()).unwrap();
    }

    #[test]
    fn ser() {
        let js_result = JSResult::Ok {
            data: DivinationCardsSample::default(),
        };

        let json = serde_json::to_string(&js_result).unwrap();

        let mut file = File::create("ser.json").unwrap();
        file.write_all(&json.as_bytes()).unwrap();
    }
}
