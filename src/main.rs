use static_init::dynamic;

mod crawler;
mod movies;

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
static mut CODES: Vec<String> = movies::Movie::codes();

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let tv_shows = tokio::spawn(movies::collect_movies(&*TVSHOWSURL, movies::MovieType::Tvshow, Some(1)));
    let movies = tokio::spawn(movies::collect_movies(&*MOVIESURL, movies::MovieType::Movie, Some(1)));

    let _ = tokio::join!(tv_shows, movies);
    
    Ok(())
}