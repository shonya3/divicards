use serde::{Deserialize, Serialize};

use crate::error::Error;

fn parse<T: for<'de> Deserialize<'de>>(s: &str) -> Vec<T> {
    let mut rdr = csv::Reader::from_reader(s.as_bytes());
    rdr.deserialize::<T>()
        .into_iter()
        .filter_map(|result| result.ok())
        .collect::<Vec<T>>()
}

fn serialize<T: Serialize + Sized>(records: Vec<T>) -> Result<String, Error> {
    let mut writer = csv::Writer::from_writer(vec![]);
    for record in records {
        writer.serialize(record)?;
    }
    let content_string = String::from_utf8(writer.into_inner().expect("Error with csv serialize"))?;
    Ok(content_string)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    static TEST_CSV: &'static str = "field1,field2\nvalue1,value2\n";

    #[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
    struct MyRecord {
        pub field1: String,
        pub field2: String,
    }

    #[test]
    fn parse() {
        let s = String::from(TEST_CSV);
        let v = super::parse::<MyRecord>(&s);
        assert_eq!(
            v,
            vec![MyRecord {
                field1: String::from("value1"),
                field2: String::from("value2")
            }]
        );
    }

    #[test]
    fn serialize() {
        let records = vec![MyRecord {
            field1: String::from("value1"),
            field2: String::from("value2"),
        }];
        let s = super::serialize(records);
        assert_eq!(s.unwrap(), TEST_CSV);
    }
}
