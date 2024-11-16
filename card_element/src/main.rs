use card_element::DivinationCardElementData;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let card_element = DivinationCardElementData::load().await.unwrap();
    let json = serde_json::to_string(&card_element).unwrap();
    std::fs::write("card_element_data.json", &json).unwrap();
}
