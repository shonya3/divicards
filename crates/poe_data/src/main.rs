use anyhow::Result;
use clap::Parser;
use divcord::poe_data::{cards::CardsData, mapbosses::MapBoss, maps::Map, PoeData};
use divi::TradeLeague;
use poe_data::GameFiles;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dump", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    /// Extract campaign act areas from game files
    Act {
        /// Game files source: `steam[:path]`, `cdn[:patch]` or `ggpk:path`
        #[arg(short, long, default_value = "steam")]
        source: GameFiles,

        #[arg(short, long, default_value = "out")]
        output: PathBuf,
    },
    /// Extract and enrich divination cards
    Divination {
        /// Game files source: `steam[:path]`, `cdn[:patch]` or `ggpk:path`
        #[arg(short, long, default_value = "steam")]
        source: GameFiles,

        #[arg(short, long, default_value = "out")]
        output: PathBuf,

        /// poe.ninja league for price fetching
        #[arg(short, long, default_value = "Standard")]
        league: TradeLeague,
    },
    /// Extract atlas maps
    Map {
        /// Game files source: `steam[:path]`, `cdn[:patch]` or `ggpk:path`
        #[arg(short, long, default_value = "steam")]
        source: GameFiles,

        #[arg(short, long, default_value = "out")]
        output: PathBuf,
    },
    /// Extract map bosses from game files
    MapBoss {
        /// Game files source: `steam[:path]`, `cdn[:patch]` or `ggpk:path`
        #[arg(short, long, default_value = "steam")]
        source: GameFiles,

        #[arg(short, long, default_value = "out")]
        output: PathBuf,
    },
    /// Run all extractions (act, map, map-boss, divination)
    All {
        /// Game files source: `steam[:path]`, `cdn[:patch]` or `ggpk:path`
        #[arg(short, long, default_value = "steam")]
        source: GameFiles,

        #[arg(short, long, default_value = "out")]
        output: PathBuf,

        #[arg(short, long, default_value = "Standard")]
        league: TradeLeague,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Act { source, output } => {
            tokio::task::spawn_blocking(move || poe_data::act::run(&source, &output))
                .await
                .unwrap_or_else(|e| Err(anyhow::anyhow!("act task panicked: {e}")))
        }
        Command::Divination {
            source,
            output,
            league,
        } => {
            let cards_output = poe_data::cards::extract_cards(&source, league).await?;
            let cards: Vec<_> = cards_output.dict.values().cloned().collect();
            let (enriched, _item_db) = poe_data::cards::card_element_data(&cards).await?;
            std::fs::create_dir_all(&output)?;

            let cards_path = output.join("cards.json");
            let cards_data = serde_json::to_string_pretty(&cards_output)?;
            std::fs::write(&cards_path, cards_data)?;

            let elem_path = output.join("cardElementData.json");
            let elem_data = serde_json::to_string_pretty(&enriched)?;
            std::fs::write(&elem_path, elem_data)?;

            let n_enabled = enriched.len();
            let n_disabled = cards.len() - n_enabled;
            println!(
                "Done — {} cards ({:?} active, {:?} disabled)",
                cards.len(),
                n_enabled,
                n_disabled
            );
            println!("  Cards: {}", cards_path.display());
            println!("  Elements: {}", elem_path.display());
            Ok(())
        }
        Command::Map { source, output } => {
            let opened = poe_data::open_game_data(&source).await?;
            let opened = opened.clone();
            let maps: Vec<Map> = tokio::task::spawn_blocking(move || {
                let opened = opened.lock().unwrap();
                eprintln!("extracting maps...");
                let maps = poe_data::maps::extract(&opened.fs, &opened.schemas)?;
                eprintln!("  {} maps extracted", maps.len());
                Ok::<_, anyhow::Error>(maps)
            })
            .await??;

            std::fs::create_dir_all(&output)?;
            let path = output.join("maps.json");
            let data = serde_json::to_string_pretty(&maps)?;
            std::fs::write(&path, data)?;
            println!("  Maps: {}", path.display());
            Ok(())
        }
        Command::MapBoss { source, output } => {
            let opened = poe_data::open_game_data(&source).await?;
            let opened = opened.clone();
            let bosses: Vec<MapBoss> = tokio::task::spawn_blocking(move || {
                let opened = opened.lock().unwrap();
                eprintln!("extracting map bosses...");
                let bosses = poe_data::mapbosses::extract(&opened.fs, &opened.schemas)?;
                eprintln!("  {} bosses extracted", bosses.len());
                Ok::<_, anyhow::Error>(bosses)
            })
            .await??;

            std::fs::create_dir_all(&output)?;
            let path = output.join("mapBosses.json");
            let data = serde_json::to_string_pretty(&bosses)?;
            std::fs::write(&path, data)?;
            println!("  Map bosses: {}", path.display());
            Ok(())
        }
        Command::All {
            source,
            output,
            league,
        } => {
            let opened = poe_data::open_game_data(&source).await?;
            let opened = opened.clone();
            let (acts, maps, bosses) = tokio::task::spawn_blocking(move || {
                let opened = opened.lock().unwrap();

                println!("=== Act Areas ===");
                let (acts, _) = poe_data::act::extract_areas(&opened.fs, &opened.schemas)?;
                println!("  {} areas -> acts.json", acts.len());

                println!("\n=== Maps ===");
                let maps: Vec<Map> = poe_data::maps::extract(&opened.fs, &opened.schemas)?;
                println!("  {} maps -> maps.json", maps.len());

                println!("\n=== Map Bosses ===");
                let bosses: Vec<MapBoss> =
                    poe_data::mapbosses::extract(&opened.fs, &opened.schemas)?;
                println!("  {} bosses -> mapBosses.json", bosses.len());

                Ok::<_, anyhow::Error>((acts, maps, bosses))
            })
            .await??;

            std::fs::create_dir_all(&output)?;
            std::fs::write(
                output.join("acts.json"),
                serde_json::to_string_pretty(&acts)?,
            )?;
            std::fs::write(
                output.join("maps.json"),
                serde_json::to_string_pretty(&maps)?,
            )?;
            std::fs::write(
                output.join("mapBosses.json"),
                serde_json::to_string_pretty(&bosses)?,
            )?;

            println!("\n=== Divination Cards ===");
            let cards_output: CardsData = poe_data::cards::extract_cards(&source, league).await?;
            let cards: Vec<_> = cards_output.dict.values().cloned().collect();
            let (enriched, _) = poe_data::cards::card_element_data(&cards).await?;
            std::fs::write(
                output.join("cards.json"),
                serde_json::to_string_pretty(&cards_output)?,
            )?;
            std::fs::write(
                output.join("cardElementData.json"),
                serde_json::to_string_pretty(&enriched)?,
            )?;
            println!("  {} cards -> cards.json", cards.len());

            println!("\n=== Composing poeData.json ===");
            let poedata = PoeData {
                acts,
                cards: cards_output,
                maps,
                mapbosses: bosses,
            };
            std::fs::write(
                output.join("poeData.json"),
                serde_json::to_string_pretty(&poedata)?,
            )?;
            println!("  poeData.json");

            println!("\nAll done.");
            Ok(())
        }
    }
}
