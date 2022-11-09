use std::collections::HashMap;
use static_init::dynamic;
use std::error;

mod crawler;
mod movies;
mod objectid;

#[dynamic]
static CLIENT: reqwest::Client = reqwest::Client::new();
static DOMAIN: &str = "https://sflix.to/";
static DB: &str = "./DB/";
#[dynamic]
static MOVIESPATH: String = format!("{DB}movies/");
#[dynamic]
static MOVIESERVERSURL: String = format!("{DOMAIN}ajax/movie/episodes/");
#[dynamic]
static SEASONSURL: String = format!("{DOMAIN}ajax/v2/tv/seasons/");
#[dynamic]
static EPISODESURL: String = format!("{DOMAIN}ajax/v2/season/episodes/");
#[dynamic]
static EPISODESERVERSURL: String = format!("{DOMAIN}ajax/v2/episode/servers/");
#[dynamic]
static MOVIESURL: String = format!("{DOMAIN}movie/");
#[dynamic]
static TVSHOWSURL: String = format!("{DOMAIN}tv-show/");
#[dynamic]
static mut CODES: HashMap<String, [u8; 12]> = movies::Movie::codes().unwrap();

type Result<T> = std::result::Result<T, Box<dyn error::Error  + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let time = std::time::SystemTime::now();
    let tv_shows = tokio::spawn(movies::collect_movies(
        &*TVSHOWSURL,
        movies::MovieType::Tvshow,
        None,
    ));
    let movies = tokio::spawn(movies::collect_movies(
        &*MOVIESURL,
        movies::MovieType::Movie,
        None,
    ));

    let _ = tokio::join!(tv_shows, movies);

    println!("{:#?}", time.elapsed().unwrap());

    Ok(())
}
