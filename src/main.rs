use crate::models::JishoResponse;

mod models;

#[tokio::main]
async fn main() {
    match JishoResponse::search("ohayou").await {
        Ok(res) => {
            for word in res.data {
                println!("Word: {}", word.slug);

                if let Some(sense) = word.senses.first() {
                    println!("Meaning: {:?}", sense.english_definitions);
                }
                println!("---");
            }
        }
        Err(e) => eprint!("Błąd: {}", e),
    }
}
