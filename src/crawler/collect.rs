pub fn get_movies(html: &String) -> Vec<(String, String, String, String)> {
    let document = Document::new(html);
    let documents = document.get_by_class(&"film_list-wrap".to_owned()).expect("could not find document").get_all_by_class(&"flw-item".to_owned());

    let mut movies: Vec<(String, String, String, String)> = Vec::new();

    for document in documents {
        let film_name = document.get_by_class(&"film-detail".to_owned()).unwrap().get_by_class(&"film-name".to_owned()).unwrap();
        let poster = document.get_by_class(&"film-poster".to_owned()).unwrap();
        let page_url = "https://sflix.to".to_owned() + &film_name.child_attribute(&"href".to_owned()).unwrap();
        let split: Vec<&str> = page_url.split("-").collect();
        let code = split[split.len()-1].to_owned();
        let name = html_escape::decode_html_entities(&film_name.child_attribute(&"title".to_owned()).unwrap()).to_string();
        let image_url = poster.child_attribute(&"data-src".to_owned()).unwrap();
        movies.push((name, page_url, code, image_url));
    }

    movies
}

pub async fn get_pages_length(url: String) -> usize {
    let data = get_request(url).await.unwrap();
    let document = Document::new(&data);
    let attribute = Attribute{key: "title".to_owned(), value: Some("Last".to_owned())};
    let mut link = document.get_by_attribute(&attribute).unwrap().child_attribute(&"href".to_owned()).unwrap();
    link = link.replace("/movie?page=", "");
    link.parse::<usize>().unwrap()
}