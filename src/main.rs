use static_init::dynamic;

mod crawler;
mod movies;

#[dynamic]
static CLIENT: reqwest::Client = reqwest::Client::new();

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let movie = movies::Movie::default();
    movie.save();
    let json = serde_json::to_string(&movie).unwrap();
    let value: movies::Movie = serde_json::from_str(&json).unwrap();
    println!("{:#?}", value);
    
    Ok(())
}