pub async fn collect_movies(url: &String) {
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
        pool.push(tokio::spawn(Movie::crawl(tuple, MovieType::Movie)));
    }

    for handle in pool {
        movies.push(handle.await.unwrap());
    }
}