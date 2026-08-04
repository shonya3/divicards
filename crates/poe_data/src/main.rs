use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use divcord::poe_data::{cards::CardsData, mapbosses::MapBoss, maps::Map, PoeData};
use divi::TradeLeague;

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
        #[arg(short, long)]
        steam: Option<PathBuf>,

        #[arg(short, long, default_value = "out")]
        output: PathBuf,
    },
    /// Extract and enrich divination cards
    Divination {
        #[arg(short, long)]
        steam: Option<PathBuf>,

        #[arg(short, long, default_value = "out")]
        output: PathBuf,

        /// poe.ninja league for price fetching
        #[arg(short, long, default_value = "Standard")]
        league: TradeLeague,
    },
    /// Extract atlas maps
    Map {
        #[arg(short, long)]
        steam: Option<PathBuf>,

        #[arg(short, long, default_value = "out")]
        output: PathBuf,
    },
    /// Extract map bosses from game files
    MapBoss {
        #[arg(short, long)]
        steam: Option<PathBuf>,

        #[arg(short, long, default_value = "out")]
        output: PathBuf,
    },
    /// Run all extractions (act, map, map-boss, divination)
    All {
        #[arg(short, long)]
        steam: Option<PathBuf>,

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
        Command::Act { steam, output } => {
            let steam = steam.unwrap_or_else(poe_data::default_steam_path);
            poe_data::act::run(&steam, &output)
        }
        Command::Divination {
            steam,
            output,
            league,
        } => {
            let steam = steam.unwrap_or_else(poe_data::default_steam_path);
            let cards_output = poe_data::cards::extract_cards(&steam, league).await?;
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
        Command::Map { steam, output } => {
            let steam = steam.unwrap_or_else(poe_data::default_steam_path);
            let (fs, schemas) = poe_data::open_game_data(&steam).await?;
            eprintln!("extracting maps...");
            let maps: Vec<Map> = poe_data::maps::extract(&fs, &schemas).await?;
            eprintln!("  {} maps extracted", maps.len());

            std::fs::create_dir_all(&output)?;
            let path = output.join("maps.json");
            let data = serde_json::to_string_pretty(&maps)?;
            std::fs::write(&path, data)?;
            println!("  Maps: {}", path.display());
            Ok(())
        }
        Command::MapBoss { steam, output } => {
            let steam = steam.unwrap_or_else(poe_data::default_steam_path);
            let (fs, schemas) = poe_data::open_game_data(&steam).await?;
            eprintln!("extracting map bosses...");
            let bosses: Vec<MapBoss> = poe_data::mapbosses::extract(&fs, &schemas)?;
            eprintln!("  {} bosses extracted", bosses.len());

            std::fs::create_dir_all(&output)?;
            let path = output.join("mapBosses.json");
            let data = serde_json::to_string_pretty(&bosses)?;
            std::fs::write(&path, data)?;
            println!("  Map bosses: {}", path.display());
            Ok(())
        }
        Command::All {
            steam: steam_opt,
            output,
            league,
        } => {
            let steam = steam_opt.unwrap_or_else(poe_data::default_steam_path);
            let (fs, schemas) = poe_data::open_game_data(&steam).await?;

            println!("=== Act Areas ===");
            let (acts, _) = poe_data::act::extract_areas(&fs, &schemas)?;
            std::fs::create_dir_all(&output)?;
            std::fs::write(
                output.join("acts.json"),
                serde_json::to_string_pretty(&acts)?,
            )?;
            println!("  {} areas -> acts.json", acts.len());

            println!("\n=== Maps ===");
            let maps: Vec<Map> = poe_data::maps::extract(&fs, &schemas).await?;
            std::fs::write(
                output.join("maps.json"),
                serde_json::to_string_pretty(&maps)?,
            )?;
            println!("  {} maps -> maps.json", maps.len());

            println!("\n=== Map Bosses ===");
            let bosses: Vec<MapBoss> = poe_data::mapbosses::extract(&fs, &schemas)?;
            std::fs::write(
                output.join("mapBosses.json"),
                serde_json::to_string_pretty(&bosses)?,
            )?;
            println!("  {} bosses -> mapBosses.json", bosses.len());

            println!("\n=== Divination Cards ===");
            let cards_output: CardsData = poe_data::cards::extract_cards(&steam, league).await?;
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
