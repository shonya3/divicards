use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub fn divcord_wasm_pkg() {
    let divcord_wasm_dir = Path::new("../divcord_wasm");
    if !divcord_wasm_dir.exists() {
        panic!("Divcord wasm dir not exists");
    }

    let output = Command::new("wasm-pack")
        .args(&["build", "--target", "web"])
        .current_dir(Path::new("../divcord_wasm"))
        .output()
        .expect("Could not generate divcord_wasm. Did you install wasm-pack?");

    if output.status.success() {
        println!("Divcord_wasm generated successfully!");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", stderr);
    }
}

pub async fn card_element<P: AsRef<Path>>(dir: Option<P>, filename: Option<&str>) {
    let card_element_data = card_element::fetch().await.unwrap();
    // let dir = dir.unwrap_or(std::env::current_dir().unwrap());

    let dir = match dir {
        Some(p) => p.as_ref().to_path_buf(),
        None => std::env::current_dir().unwrap(),
    };

    let filename = filename.unwrap_or("cardElementJson.json");
    let p = dir.join(filename);

    std::fs::write(p, serde_json::to_string(&card_element_data).unwrap()).unwrap();
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    divcord_wasm_pkg();
    card_element(Option::<PathBuf>::None, None).await;
}
