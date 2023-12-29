use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(err) => err.fmt(f),
            Error::SerdeError(err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::SerdeError(value)
    }
}

#[allow(async_fn_in_trait)]
pub trait DataLoader<T, E>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
    E: From<Error>,
{
    fn filename() -> &'static str;
    fn reload(&self) -> bool {
        true
    }
    async fn fetch(&self) -> Result<T, E>;

    fn file_path(&self) -> PathBuf {
        let dir = std::env::current_dir().unwrap().join("data");
        if !dir.exists() {
            std::fs::create_dir(&dir).unwrap();
        }

        dir.join(Self::filename())
    }
    async fn update(&self) -> Result<(), E> {
        let t = self.fetch().await?;
        self.save(&t)?;
        Ok(())
    }
    fn save(&self, data: &T) -> Result<(), E> {
        let json = serde_json::to_string(data).map_err(Error::SerdeError)?;
        fs::write(self.file_path(), &json).map_err(Error::IoError)?;

        Ok(())
    }
    async fn load(&self) -> Result<T, E> {
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
            let file = File::open(&self.file_path()).map_err(Error::IoError)?;
            let reader = BufReader::new(file);

            Ok(serde_json::from_reader(reader).map_err(Error::SerdeError)?)
        } else {
            let t = self.fetch().await?;
            self.save(&t)?;
            Ok(t)
        }
    }
}
