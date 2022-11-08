pub fn get_servers(html: &String) -> Result<Vec<(String, String)>> {
    let documents = Document::new(html).get_all_by_name(&"a".to_owned());
    let mut servers: Vec<(String, String)> = Vec::new();

    for document in documents {
        let id = match document.child_attribute(&"data-id".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get data-id for server".to_owned()))?
        };
        let span = match document.get_by_name(&"span".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get span for server".to_owned()))?
        };
        let name = match span.get_text() {
            Some(value) => value,
            None => return Err(Error("Error: could not get server name".to_owned()))?
        };
        servers.push((name, id));
    }

    Ok(servers.into_iter().rev().collect::<Vec<(String, String)>>())
}

pub fn get_seasons(html: &String) -> Result<Vec<(String, String)>> {
    let documents = Document::new(html).get_all_by_name(&"a".to_owned());
    let mut seasons: Vec<(String, String)> = Vec::new();

    for document in documents {
        let code = match document.attribute(&"data-id".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get data id for seasons".to_owned()))?
        };
        let name_div = match document.get_by_name(&"a".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get a's for seasons".to_owned()))?
        };
        let name = match name_div.get_text() {
            Some(value) => value,
            None => return Err(Error("Error: could not get text for server's name_div".to_owned()))?
        };
        seasons.push((name, code));
    }

    Ok(seasons.into_iter().rev().collect::<Vec<(String, String)>>())
}

pub fn get_episodes(html: &String) -> Result<Vec<(i32, String, String, String)>> {
    let documents = Document::new(html).get_all_by_class(&"swiper-slide".to_owned());

    let mut episodes: Vec<(i32, String, String, String)> = Vec::new();

    for document in documents {
        let image = match document.get_by_name(&"img".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get img div for episodes".to_owned()))?
        };
        let image_url = match image.attribute(&"src".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get img's src for episodes".to_owned()))?
        };
        let code = match document.child_attribute(&"data-id".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get data-id for episodes".to_owned()))?
        };
        let episode = match document.get_by_class(&"episode-number".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get episode number".to_owned()))?
        };
        let number_string = match episode.get_text() {
            Some(value) => value,
            None => return Err(Error("Error: could not get episode_number_string".to_owned()))?
        };
        let number = number_string.replace(":", "").replace("Episode ", "").replace("\n", "").parse::<i32>()?;
        let film_name = match document.get_by_class(&"film-name".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get film-name for episodes".to_owned()))?
        };
        let name = match film_name.child_attribute(&"title".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get film-name title for episodes".to_owned()))?
        };
        episodes.push((number, name, code, image_url));
    }

    Ok(episodes.into_iter().rev().collect::<Vec<(i32, String, String, String)>>())
}