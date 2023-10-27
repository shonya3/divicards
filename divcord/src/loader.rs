use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

use async_trait::async_trait;

use crate::error::Error;

#[async_trait]
pub trait DataLoader<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    fn filename(&self) -> &'static str;
    fn reload(&self) -> bool {
        true
    }
    async fn fetch(&self) -> Result<T, Error>;

    fn file_path(&self) -> PathBuf {
        let dir = std::env::current_dir().unwrap().join("data");
        if !dir.exists() {
            std::fs::create_dir(&dir).unwrap();
        }

        dir.join(self.filename())
    }
    async fn update(&self) -> Result<(), Error> {
        let t = self.fetch().await?;
        self.save(&t)?;
        Ok(())
    }
    fn save(&self, data: &T) -> Result<(), Error> {
        let json = serde_json::to_string(data)?;
        fs::write(self.file_path(), &json)?;

        Ok(())
    }
    async fn load(&self) -> Result<T, Error> {
        let path = self.file_path();
        let exists = path.try_exists().unwrap();
        let file_days_old = || -> Option<f32> {
            pub const DAY_AS_SECS: f64 = 86_400.0;
            match exists {
                true => match fs::metadata(&path) {
                    Ok(metadata) => match metadata.modified() {
                        Ok(time) => {
                            let days =
                                (time.elapsed().unwrap().as_secs() as f64 / DAY_AS_SECS) as f32;
                            Some(days)
                        }
                        Err(_) => None,
                    },
                    Err(_) => None,
                },
                false => None,
            }
        };

        let up_to_date = || -> bool {
            match file_days_old() {
                Some(n) if n <= 1.0 => true,
                _ => false,
            }
        };

        if up_to_date() || (exists && self.reload() == false) {
            let file = File::open(&self.file_path())?;
            let reader = BufReader::new(file);

            Ok(serde_json::from_reader(reader)?)
        } else {
            let t = self.fetch().await?;
            self.save(&t)?;
            Ok(t)
        }
    }
}
