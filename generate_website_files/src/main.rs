use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use card_element::DivinationCardElementData;
use divcord::table::DivcordTable;
use poe_data::PoeData;
use serde::Serialize;

#[tokio::main]
async fn main() {
    let dir = PathBuf::from("../../divicards-site/src/gen");
    if !dir.exists() {
        std::fs::create_dir_all(&dir).unwrap();
    }

    divcord_wasm_pkg(&dir, "divcordWasm");

    let divcord_table = DivcordTable::load().await.unwrap();
    let poe_data = PoeData::load().await.unwrap();
    let card_element = DivinationCardElementData::load().await.unwrap();
    let records = divcord_table.sourceful_records(&poe_data).unwrap();

    write(&records, &dir, "records.json");
    write(&poe_data, &dir, PoeData::filename());
    write(&card_element, &dir, DivinationCardElementData::filename());
    std::fs::write(
        dir.join("ISource.interface.ts"),
        &divcord::dropsource::Source::typescript_types(),
    )
    .unwrap();
}

pub fn divcord_wasm_pkg(path: &Path, dirname: &str) {
    let dir_path = Path::new("../divcord_wasm");

    let path = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("cli")
        .join(path)
        .join(dirname);
    println!("{}", path.display());

    if dir_path.exists() && dir_path.is_dir() {
        let output = Command::new("wasm-pack")
            .args(&[
                "build",
                "--target",
                "web",
                "--out-dir",
                &path.display().to_string(),
            ])
            .current_dir(&dir_path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("Command executed successfully!");
                } else {
                    eprintln!("Error executing command");
                }
            }
            Err(err) => {
                eprintln!("Failed to execute command: {}", err);
            }
        }
    } else {
        eprintln!("The directory does not exist or is not a directory.");
    }
}

pub async fn card_element(config: &Config) {
    let card_element_data = card_element::fetch().await.unwrap();
    // let dir = dir.unwrap_or(std::env::current_dir().unwrap());

    let p = config.dir.join(&config.filename);

    std::fs::write(p, serde_json::to_string(&card_element_data).unwrap()).unwrap();
}

pub struct Config {
    pub dir: PathBuf,
    pub filename: String,
}

impl Config {
    pub const fn new(dir: PathBuf, filename: String) -> Self {
        Self { dir, filename }
    }

    pub fn with_current_dir(filename: String) -> Self {
        Self {
            dir: Default::default(),
            filename,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dir: Default::default(),
            filename: Default::default(),
        }
    }
}

pub fn write<T>(value: &T, dir: &Path, filename: &str)
where
    T: Serialize,
{
    let json = serde_json::to_string(&value).unwrap();
    let p = dir.join(filename);
    std::fs::write(p, &json).unwrap();
}
