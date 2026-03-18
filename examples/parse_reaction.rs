// examples/parse_reaction.rs
use gene_sys::parcing::reaction::function_reaction;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reaction_id = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "R00259".to_string());
    
    let url = format!("https://www.kegg.jp/entry/{}", reaction_id);
    println!("Загружаем реакцию: {}", reaction_id);

    let client = reqwest::Client::builder()
        .user_agent("curl/7.68.0")
        .build()?;

    let response = client.get(&url).send().await?;
    let html = response.text().await?;

    match function_reaction(html) {
        Some(reaction) => {
            println!("\nУспешно распарсено:");
            println!("{:#?}", reaction);
        }
        None => {
            println!("\nНе удалось распарсить {}", reaction_id);
        }
    }

    Ok(())
}