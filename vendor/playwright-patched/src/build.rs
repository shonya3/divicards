use std::{
    env, fmt, fs,
    io::{self, Write},
    path::{Path, PathBuf, MAIN_SEPARATOR},
};

const DRIVER_VERSION: &str = "1.11.0";

fn main() {
    let out_dir: PathBuf = env::var_os("OUT_DIR").unwrap().into();
    let dest = out_dir.join("driver.zip");
    let platform = PlaywrightPlatform::default();
    fs::write(out_dir.join("platform"), platform.to_string()).unwrap();

    #[cfg(not(feature = "only-for-docs-rs"))]
    assemble_driver(&dest);

    println!("cargo:rerun-if-changed=src/build.rs");
    println!("cargo:rustc-env=SEP={}", MAIN_SEPARATOR);
}

#[cfg(not(feature = "only-for-docs-rs"))]
fn assemble_driver(dest: &Path) {
    let cache_dir: &Path = "/tmp/build-playwright-rust".as_ref();
    let cached = cache_dir.join("driver.zip");

    if cached.exists() {
        let size = cached.metadata().map(|m| m.len()).unwrap_or(0);
        if size > 100_000 {
            fs::copy(cached, dest).unwrap();
            check_size(dest);
            return;
        }
    }

    let tmp = env::temp_dir().join(format!("playwright-assembly-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();

    let npm_tgz = download_npm_package(DRIVER_VERSION);
    let package_dir = tmp.join("package");
    extract_tgz(&npm_tgz, &tmp);

    let cli_js = package_dir.join("lib").join("cli").join("cli.js");
    assert!(
        cli_js.exists(),
        "npm package missing cli.js at {}",
        cli_js.display()
    );

    let sh_path = tmp.join("playwright.sh");
    {
        let mut sh = fs::File::create(&sh_path).unwrap();
        writeln!(sh, "#!/bin/bash").unwrap();
        writeln!(sh, "dir=\"$(cd \"$(dirname \"$0\")\" && pwd)\"").unwrap();
        writeln!(sh, "exec node \"$dir/package/lib/cli/cli.js\" \"$@\"").unwrap();
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&sh_path, fs::Permissions::from_mode(0o755)).unwrap();
    }

    install_npm_deps(&package_dir);

    let driver_tmp = tmp.join("driver-tmp");
    fs::create_dir_all(&driver_tmp).unwrap();

    let dest_package = driver_tmp.join("package");
    if dest_package.exists() {
        fs::remove_dir_all(&dest_package).unwrap();
    }
    copy_dir(&package_dir, &dest_package);

    let dest_sh = driver_tmp.join("playwright.sh");
    fs::copy(&sh_path, &dest_sh).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&dest_sh, fs::Permissions::from_mode(0o755)).unwrap();
    }

    create_zip(&driver_tmp, dest);

    fs::create_dir_all(cache_dir).unwrap();
    let _ = fs::copy(dest, cached);

    check_size(dest);
    let _ = fs::remove_dir_all(&tmp);
}

#[cfg(not(feature = "only-for-docs-rs"))]
fn download_npm_package(version: &str) -> Vec<u8> {
    let url = format!("https://registry.npmjs.org/playwright-core/-/playwright-core-{version}.tgz");
    eprintln!("Downloading playwright-core v{version} from npm registry...");
    let resp = reqwest::blocking::get(&url)
        .unwrap_or_else(|e| panic!("Failed to download {}: {}", url, e));
    let status = resp.status();
    assert!(
        status.is_success(),
        "Download of {} failed with status {}",
        url,
        status
    );
    let bytes = resp.bytes().unwrap().to_vec();
    eprintln!("Downloaded {} bytes", bytes.len());
    bytes
}

#[cfg(not(feature = "only-for-docs-rs"))]
fn extract_tgz(tgz: &[u8], dest: &Path) {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let decoder = GzDecoder::new(tgz);
    let mut archive = Archive::new(decoder);
    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();
        if let Ok(path) = entry.path() {
            if path.starts_with("package") {
                entry.unpack_in(dest).unwrap();
            }
        }
    }
    assert!(
        dest.join("package").exists(),
        "npm tarball extraction failed: no package/ directory"
    );
}

#[cfg(not(feature = "only-for-docs-rs"))]
fn install_npm_deps(package_dir: &Path) {
    eprintln!("Installing npm dependencies in {}...", package_dir.display());
    let status = std::process::Command::new("npm")
        .args(&[
            "install",
            "--production",
            "--no-audit",
            "--no-fund",
            "--no-save",
            "--prefix",
        ])
        .arg(package_dir)
        .current_dir(package_dir)
        .status()
        .unwrap_or_else(|e| panic!("Failed to run npm install in {}: {}", package_dir.display(), e));
    assert!(
        status.success(),
        "npm install failed with exit code {}",
        status
    );
    eprintln!("npm dependencies installed.");
}

#[cfg(not(feature = "only-for-docs-rs"))]
fn create_zip(src_dir: &Path, dest: &Path) {
    use zip::write::FileOptions;
    use zip::ZipWriter;

    let file = fs::File::create(dest).unwrap();
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    add_dir_to_zip(&mut zip, src_dir, src_dir, &options).unwrap();
    zip.finish().unwrap();

    eprintln!(
        "Created driver zip at {} ({} bytes)",
        dest.display(),
        dest.metadata().map(|m| m.len()).unwrap_or(0)
    );
}

#[cfg(not(feature = "only-for-docs-rs"))]
fn add_dir_to_zip(
    zip: &mut zip::ZipWriter<fs::File>,
    base: &Path,
    dir: &Path,
    options: &zip::write::FileOptions,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .strip_prefix(base)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        if path.is_dir() {
            zip.add_directory(&name, *options).unwrap();
            add_dir_to_zip(zip, base, &path, options)?;
        } else {
            zip.start_file(&name, *options).unwrap();
            let mut f = fs::File::open(&path)?;
            io::copy(&mut f, zip)?;
        }
    }
    Ok(())
}

#[cfg(not(feature = "only-for-docs-rs"))]
fn copy_dir(src: &Path, dst: &Path) {
    if !dst.exists() {
        fs::create_dir_all(dst).unwrap();
    }
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir(&src_path, &dst_path);
        } else {
            fs::copy(&src_path, &dst_path).unwrap();
        }
    }
}

fn check_size(p: &Path) {
    let size = p.metadata().map(|m| m.len()).unwrap_or(0);
    assert!(size > 100_000, "driver zip is too small: {} bytes", size);
}

#[cfg(feature = "only-for-docs-rs")]
fn download_npm_package(_version: &str) -> Vec<u8> {
    Vec::new()
}

#[derive(Clone, Copy)]
enum PlaywrightPlatform {
    Linux,
    Win32,
    Win32x64,
    Mac,
}

impl fmt::Display for PlaywrightPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Linux => write!(f, "linux"),
            Self::Win32 => write!(f, "win32"),
            Self::Win32x64 => write!(f, "win32_x64"),
            Self::Mac => write!(f, "mac"),
        }
    }
}

impl Default for PlaywrightPlatform {
    fn default() -> Self {
        match env::var("CARGO_CFG_TARGET_OS").as_deref() {
            Ok("linux") => return PlaywrightPlatform::Linux,
            Ok("macos") => return PlaywrightPlatform::Mac,
            _ => (),
        };
        if env::var("CARGO_CFG_WINDOWS").is_ok() {
            if env::var("CARGO_CFG_TARGET_POINTER_WIDTH").as_deref() == Ok("64") {
                PlaywrightPlatform::Win32x64
            } else {
                PlaywrightPlatform::Win32
            }
        } else if env::var("CARGO_CFG_UNIX").is_ok() {
            PlaywrightPlatform::Linux
        } else {
            panic!("Unsupported plaform");
        }
    }
}
