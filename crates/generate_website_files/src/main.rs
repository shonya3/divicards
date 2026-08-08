mod avatars;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use card_element::DivinationCardElementData;
use divcord::{spreadsheet::Spreadsheet, ParseRecordError, PoeData, Record, Source};
use fs_cache_fetcher::DataFetcher;
use poe_data::fetchers::{CardElementsFetcher, PoeDataFetcher};
use serde::Serialize;

// cargo install cargo-binstall
// cargo binstall wasm-pack
#[tokio::main]
async fn main() {
    // Load poeData (runs the extraction pipeline when stale)
    let poe_data: PoeData = PoeDataFetcher::default().load().await.unwrap();

    // Load cardElementData
    let card_element: Vec<DivinationCardElementData> =
        CardElementsFetcher::default().load().await.unwrap();

    let dir = project_root::get_project_root()
        .unwrap()
        .parent()
        .unwrap()
        .join("divicards-site")
        .join("gen");
    println!("target dir: {}", dir.display());

    let json_dir = dir.join("json");
    if !json_dir.exists() {
        std::fs::create_dir_all(&json_dir).unwrap();
    }

    ensure_all_unique_rewards_handled(&card_element).unwrap();
    write(
        &card_element,
        &json_dir,
        CardElementsFetcher::default().filename(),
    );

    // Sync cardElementData to poe-custom-elements (pretty-printed)
    let poe_custom_elements_dir = project_root::get_project_root()
        .unwrap()
        .parent()
        .unwrap()
        .join("poe-custom-elements")
        .join("src")
        .join("elements")
        .join("divination-card");
    println!("target dir: {}", poe_custom_elements_dir.display());
    write_pretty(
        &card_element,
        &poe_custom_elements_dir,
        CardElementsFetcher::default().filename(),
    );

    // ── Spreadsheet (still fetched live) ──────────────────────
    dotenv::dotenv().ok();
    let spreadsheet = Spreadsheet::load().await.unwrap();
    let records = parse_divcord_records(&spreadsheet, &poe_data);

    if !dir.exists() {
        panic!(
            "divicards-site/gen dir does not exist at path: {}",
            dir.display()
        );
    }

    let mut sources_hashmap: HashMap<String, Source> = records
        .clone()
        .into_iter()
        .flat_map(|record| record.sources.into_iter().chain(record.verify_sources))
        .collect::<HashSet<Source>>()
        .into_iter()
        .map(|source| (source.slug(), source))
        .collect();

    poe_data.maps.iter().for_each(|map| {
        sources_hashmap
            .entry(map.slug.clone())
            .or_insert(Source::Map(map.name.clone()));
    });

    write(&sources_hashmap, &json_dir, "sources2.json");
    write(&records, &json_dir, "records.json");
    write(&poe_data, &json_dir, PoeDataFetcher::default().filename());

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
    let dir_path = project_root::get_project_root()
        .unwrap()
        .join("crates")
        .join("divcord_wasm");

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
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if !stderr.is_empty() {
                        eprintln!("{stderr}"); // Print wasm-pack's stderr
                    }
                    println!("Command executed successfully!");
                } else {
                    eprintln!("Error executing command. Status: {}", output.status);
                    eprintln!(
                        "--- stderr ---\n{}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                    eprintln!(
                        "--- stdout ---\n{}",
                        String::from_utf8_lossy(&output.stdout)
                    );
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

pub fn write_pretty<T>(value: &T, dir: &Path, filename: &str)
where
    T: Serialize,
{
    let json = serde_json::to_string_pretty(&value).unwrap();
    let p = dir.join(filename);
    std::fs::write(p, json).unwrap();
}

/// Ensure that all card elements that have unique class in their reward html
///  also have something in "unique" field.
pub fn ensure_all_unique_rewards_handled(
    card_elements: &[DivinationCardElementData],
) -> Result<(), String> {
    let cards = card_elements
        .iter()
        .filter(|c| c.reward_html.contains("unique") && c.unique.is_none())
        .collect::<Vec<_>>();

    if !cards.is_empty() {
        let cards_s = cards
            .iter()
            .map(|card| format!("{}: {}", card.name, card.reward_html))
            .collect::<Vec<_>>()
            .join("\n");

        return Err(format!(
            "Card element data preparation error. Not all unique rewards are handled.\n{cards_s}"
        ));
    }

    Ok(())
}
