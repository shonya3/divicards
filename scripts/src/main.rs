pub mod act;
pub mod card_element;
pub mod cards;
pub mod consts;
pub mod dropsource;
pub mod error;
pub mod maps;
pub mod reward;
pub mod scripts;
pub mod table;
pub mod table_record;

#[allow(unused)]
use crate::scripts::{read_original_table_sheet, update_all_jsons};
#[allow(unused)]
use crate::table::Table;
#[allow(unused)]
use divi::consts::CARDS;
#[allow(unused)]
use dropsource::Source;
#[allow(unused)]
use error::Error;
#[allow(unused)]
use scraper::node::Text;
#[allow(unused)]
use scraper::Element;
#[allow(unused)]
use scraper::{ElementRef, Html, Selector};
#[allow(unused)]
use scripts::read_table;
#[allow(unused)]
use serde::{Deserialize, Serialize};
#[allow(unused)]
use serde_json::{json, Value};
#[allow(unused)]
use std::collections::HashSet;
#[allow(unused)]
use std::env;
#[allow(unused)]
use std::fs;
#[allow(unused)]
use std::path::Path;
#[allow(unused)]
use std::str::FromStr;
#[allow(unused)]
use std::{collections::HashMap, fmt::Display, slice::Iter};
#[allow(unused)]
use table_record::{CardDropTableRecord, Confidence, GreyNote};

#[tokio::main]
async fn main() {
    update_all_jsons().await;
}
