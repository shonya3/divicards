mod avatars;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use card_element::DivinationCardElementData;
use divcord::{spreadsheet::Spreadsheet, ParseRecordError, Record, Source};
use poe_data::PoeData;
use serde::Serialize;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // load and parse
    let spreadsheet = Spreadsheet::load().await.unwrap();
    let poe_data = PoeData::load().await.unwrap();

    let records = parse_divcord_records(&spreadsheet, &poe_data);

    let card_element = DivinationCardElementData::load().await.unwrap();

    // 1. Write data jsons
    // Prepare paths
    let dir = PathBuf::from("../../divicards-site/gen");
    let json_dir = dir.join("json");
    if !json_dir.exists() {
        std::fs::create_dir_all(&json_dir).unwrap();
    }

    let sources_hashmap: HashMap<String, Source> = records
        .clone()
        .into_iter()
        .flat_map(|record| record.sources.into_iter().chain(record.verify_sources))
        .collect::<HashSet<Source>>()
        .into_iter()
        .map(|source| (source.slug(), source))
        .collect();
    write(&sources_hashmap, &json_dir, "sources2.json");
    write(&records, &json_dir, "records.json");
    write(&poe_data, &json_dir, PoeData::filename());
    write(
        &card_element,
        &json_dir,
        DivinationCardElementData::filename(),
    );

    match avatars::prepare_avatars_ts().await {
        Ok(avatars_string) => std::fs::write(dir.join("avatars.ts"), avatars_string).unwrap(),
        Err(err) => println!("Preparing avatars error: {err:?}"),
    }

    // 2. Generate TypeScript
    std::fs::write(
        dir.join("Source.ts"),
        divcord::dropsource::Source::typescript_types(),
    )
    .unwrap();

    // 3. Compile WASM Divcord
    divcord_wasm_pkg(&dir, "divcordWasm");
}

fn parse_divcord_records(spreadsheet: &Spreadsheet, poe_data: &PoeData) -> Vec<Record> {
    println!("Parse divcord records");
    let on_err = |s: &str| {
        println!("Error parsing divcord records: {s}");
    };

    let mut records: Vec<Record> = vec![];
    for record in divcord::records_iter(spreadsheet, poe_data) {
        match record {
            Ok(record_result) => {
                records.push(record_result.record);
                if !record_result.errors.is_empty() {
                    let errors_string =
                        ParseRecordError::ParseDropSources(record_result.errors).to_string();
                    on_err(&errors_string);
                }
            }
            Err(err) => {
                on_err(&err.to_string());
            }
        }
    }
    records
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
            .args([
                "build",
                "--target",
                "web",
                "--out-dir",
                &path.display().to_string(),
            ])
            .current_dir(dir_path)
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
                eprintln!("Failed to execute command: {err}");
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

pub fn write<T>(value: &T, dir: &Path, filename: &str)
where
    T: Serialize,
{
    let json = serde_json::to_string(&value).unwrap();
    let p = dir.join(filename);
    std::fs::write(p, json).unwrap();
}
