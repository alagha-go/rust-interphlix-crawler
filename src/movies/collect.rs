pub async fn collect_movies(url: &String, movie_type: MovieType) {
    let mut tuples = Vec::new();
    let length = crawler::get_pages_length(url.clone()).await;
    
    let mut pool = Vec::new();
    
    for index in 1..length+1 {
        let url = format!("{}?page={}", url, index);
        pool.push(tokio::spawn(crawler::get_request(url)));
    }
    
    for handle in pool {
        let html = handle.await.unwrap().unwrap();
        let res =crawler::get_movies(&html);
        tuples.extend(res);
    }
    
    let mut movies = Vec::new();
    let mut pool = Vec::new();

    for tuple in tuples {
        pool.push(tokio::spawn(Movie::crawl(tuple, movie_type.clone())));
    }

    for handle in pool {
        movies.push(handle.await.unwrap());
    }

    Movie::save_codes();
}

pub async fn collect_seasons(url: &String) -> Vec<Season> {
    let html = crawler::get_request(url.to_owned()).await.unwrap();
    let tuples = crawler::get_seasons(&html);
    let mut seasons: Vec<Season> = Vec::new();

    let mut pool = Vec::new();

    for (name, code) in tuples {
        let url = format!("{}{code}", &*crate::EPISODESURL);
        pool.push(tokio::spawn(async move{
            let episodes = collect_episodes(&url).await;
            let id = ObjectId::new();
            Season{id, name, code, episodes}
        }));
    }

    for handle in pool {
        seasons.push(handle.await.unwrap());
    }
    
    seasons
}

pub async fn collect_episodes(url: &String) -> Vec<Episode> {
    let html = crawler::get_request(url.to_owned()).await.unwrap();
    let tuples = crawler::get_episodes(&html);
    let mut episodes: Vec<Episode> = Vec::new();

    for (number, name, code, image_url) in tuples {
        let url = format!("{}{code}", &*crate::EPISODESERVERSURL);
        let servers = collect_servers(&url).await;
        let id = ObjectId::new();
        episodes.push(Episode{id, number, name, code, image_url, servers});
    }

    episodes
}

pub async fn collect_servers(url: &String) -> Vec<Server> {
    let html = crawler::get_request(url.to_owned()).await.unwrap();
    let tuples = crawler::get_servers(&html);
    let mut servers: Vec<Server> = Vec::new();

    for (name, video_id) in tuples {
        let id = ObjectId::new();
        servers.push(Server{id, name, video_id});
    }

    servers
}