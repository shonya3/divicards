import { readFileSync, writeFileSync } from "fs";

const version = process.argv[2];
if (!version) {
  console.error("Usage: node bump-version.mjs <version>");
  process.exit(1);
}

const files = ["package.json", "packages/app/package.json", "packages/app/src-tauri/tauri.conf.json", "Cargo.toml"];

for (const file of files) {
  const content = readFileSync(file, "utf8");
  const updated = content
    .replace(/("version":\s*")[^"]*(")/, `$1${version}$2`)
    .replace(/^(version\s*=\s*")[^"]*(")/m, `$1${version}$2`);
  writeFileSync(file, updated);
  console.log(`${file} → ${version}`);
}
