pub fn get_movies(html: &String) -> Result<Vec<(String, String, String, String)>> {
    let document = Document::new(html);
    let documents =  match document.get_by_class(&"film_list-wrap".to_owned()) {
        Some(value) => value,
        None => return Err(Error("Error: could not get film_list-wrap".to_owned()))?
    }.get_all_by_class(&"flw-item".to_owned());

    let mut movies: Vec<(String, String, String, String)> = Vec::new();

    for document in documents {
        let film_detail = match document.get_by_class(&"film-detail".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get film-detail".to_owned()))?
        };
        let film_name = match film_detail.get_by_class(&"film-name".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get film-name".to_owned()))?
        };
        let poster = match document.get_by_class(&"film-poster".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get film-poster".to_owned()))?
        };
        let page_url = "https://sflix.to".to_owned() + match &film_name.child_attribute(&"href".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get page_url".to_owned()))?
        };
        let split: Vec<&str> = page_url.split("-").collect();
        let code = split[split.len()-1].to_owned();
        let name = html_escape::decode_html_entities(match &film_name.child_attribute(&"title".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get movie title".to_owned()))?
        }).to_string();
        let image_url = match poster.child_attribute(&"data-src".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get image_url".to_owned()))?
        };
        movies.push((name, page_url, code, image_url));
    }

    Ok(movies)
}

pub async fn get_pages_length(url: String) -> Result<usize> {
    let data = get_request(url).await?;
    let mut document = Document::new(&data);
    let attribute = Attribute{key: "title".to_owned(), value: Some("Last".to_owned())};
    document = match document.get_by_attribute(&attribute) {
        Some(value) => value,
        None => return Err(Error("Error: could not get last page div".to_owned()))?
    };
    let mut link = match document.child_attribute(&"href".to_owned()) {
        Some(value) => value,
        None => return Err(Error("Error: could not get last page div's href".to_owned()))?
    };
    link = link.replace("/movie?page=", "");
    Ok(link.parse::<usize>()?)
}