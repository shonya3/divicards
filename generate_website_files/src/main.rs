mod avatars;
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use card_element::DivinationCardElementData;
use divcord::spreadsheet::Spreadsheet;
use poe_data::PoeData;
use serde::Serialize;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // Prepare paths
    let dir = PathBuf::from("../../divicards-site/src/gen");
    let json_dir = dir.join("json");
    if !json_dir.exists() {
        std::fs::create_dir_all(&json_dir).unwrap();
    }

    // 1. Compile WASM Divcord
    divcord_wasm_pkg(&dir, "divcordWasm");

    // 2. Write data jsons
    let spreadsheet = Spreadsheet::load().await.unwrap();
    let poe_data = PoeData::load().await.unwrap();
    let card_element = DivinationCardElementData::load().await.unwrap();
    let Ok(records) = divcord::records(&spreadsheet, &poe_data) else {
        eprintln!("divcord::records parse Err. Scanning all possible errors with records_iter...");
        for result in divcord::records_iter(&spreadsheet, &poe_data) {
            if let Err(err) = result {
                eprintln!("{err:?}");
            }
        }

        std::process::exit(0);
    };

    write(&records, &json_dir, "records.json");
    write(&poe_data, &json_dir, PoeData::filename());
    write(
        &card_element,
        &json_dir,
        DivinationCardElementData::filename(),
    );
    std::fs::write(
        &dir.join("avatars.ts"),
        &avatars::prepare_avatars_ts().await,
    )
    .unwrap();

    // 3. Generate TypeScript
    std::fs::write(
        dir.join("Source.ts"),
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
        .join("generate_website_files")
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
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("{}", String::from_utf8_lossy(&output.stdout)); // Print wasm-pack's stdout
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr)); // Print wasm-pack's stderr

                    println!("Command executed successfully!");
                } else {
                    eprintln!("Error executing command");
                }
            }
            Err(err) => {
                eprintln!("Failed to execute command: {}", err);
            }
        }

        let gitignore_path = path.join(".gitignore");
        if gitignore_path.exists() {
            std::fs::remove_file(&gitignore_path).unwrap();
            println!(".gitignore is successfully deleted");
        }
    } else {
        panic!("The directory does not exist or is not a directory.");
    }
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
