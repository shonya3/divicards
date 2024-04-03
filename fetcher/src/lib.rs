use std::fmt::Display;
use std::time::{Duration, SystemTime};
use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

#[allow(async_fn_in_trait)]
pub trait DataFetcher<T, E>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
    E: From<FetcherError>,
    Self: WithConfig + Default,
{
    async fn fetch(&self) -> Result<T, E>;

    async fn load_with_mut_default_config(f: impl Fn(&mut Config)) -> Result<T, E> {
        let mut default = Self::default();
        f(&mut default.config_mut());
        default.load().await
    }

    async fn load(&self) -> Result<T, E> {
        let config = self.config();
        match up_to_date(&self.file_path(), &config.stale).unwrap_or(false) {
            true => {
                let file = File::open(&self.file_path()).map_err(FetcherError::IoError)?;
                let reader = BufReader::new(file);

                Ok(serde_json::from_reader(reader).map_err(FetcherError::SerdeError)?)
            }
            false => {
                let fetched = self.fetch().await?;
                if config.save {
                    self.save(&fetched)?;
                }
                Ok(fetched)
            }
        }
    }

    fn filename(&self) -> &'static str {
        self.config().filename
    }

    fn file_path(&self) -> PathBuf {
        let dir = project_root::get_project_root()
            .unwrap_or_else(|_| std::env::current_dir().unwrap())
            .join("data");
        if !dir.exists() {
            std::fs::create_dir_all(&dir).unwrap();
        }

        dir.join(self.filename())
    }

    fn up_to_date(&self) -> bool {
        up_to_date(&self.file_path(), &self.config().stale).unwrap_or(false)
    }

    async fn update(&self) -> Result<(), E> {
        let t = self.fetch().await?;
        self.save(&t)?;
        Ok(())
    }

    fn save(&self, data: &T) -> Result<(), E> {
        if !self.config().save {
            return Ok(());
        }

        let json = serde_json::to_string(data).map_err(FetcherError::SerdeError)?;
        fs::write(self.file_path(), &json).map_err(|err| FetcherError::IoError(err))?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct FileNotExists(PathBuf);
fn up_to_date(path: &PathBuf, stale: &Stale) -> Result<bool, FileNotExists> {
    if !path.exists() {
        return Ok(false);
    }

    match stale {
        Stale::Never => Ok(true),
        Stale::After(stale_after) => {
            let file_not_exists = |_| FileNotExists(path.to_owned());

            let last_modified = fs::metadata(&path)
                .map_err(file_not_exists)?
                .modified()
                .map_err(file_not_exists)?;
            let until = last_modified + *stale_after;
            Ok(until > SystemTime::now())
        }
        Stale::ReloadEveryTime => Ok(false),
    }
}

#[derive(Debug)]
pub enum FetcherError {
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
}

impl Display for FetcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(err) => err.fmt(f),
            Self::SerdeError(err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for FetcherError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<serde_json::Error> for FetcherError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value)
    }
}

#[derive(Default, Clone)]
pub struct Config {
    pub save: bool,
    pub filename: &'static str,
    pub stale: Stale,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    pub fn stale(&mut self, stale: Stale) -> &mut Self {
        self.stale = stale;
        self
    }

    pub fn save(&mut self, save: bool) -> &mut Self {
        self.save = save;
        self
    }

    pub fn filename(&mut self, filename: &'static str) -> &mut Self {
        self.filename = filename;
        self
    }
}

#[derive(Default, Clone)]
pub enum Stale {
    #[default]
    Never,
    After(Duration),
    ReloadEveryTime,
}

#[derive(Default)]
pub struct ConfigBuilder {
    pub save: bool,
    pub filename: &'static str,
    pub stale: Stale,
}

impl ConfigBuilder {
    pub fn stale(mut self, stale: Stale) -> Self {
        self.stale = stale;
        self
    }

    pub fn save(mut self, save: bool) -> Self {
        self.save = save;
        self
    }

    pub fn filename(mut self, filename: &'static str) -> Self {
        self.filename = filename;
        self
    }

    pub fn build(self) -> Config {
        Config {
            save: self.save,
            filename: self.filename,
            stale: self.stale,
        }
    }
}

pub trait WithConfig {
    fn config(&self) -> &Config;
    fn config_mut(&mut self) -> &mut Config;
}
