pub fn get_movie_content(html: &String) -> Result<(String, String, String, Vec<String>, Vec<String>, std::time::Duration, Vec<String>, Vec<String>)> {
    let document = Document::new(html);
    let description_div = match document.get_by_class(&"description".to_owned()) {
        Some(value) => value,
        None => return Err(Error("Error: could not get description_div".to_owned()))?
    };
    let description = match description_div.get_text() {
        Some(value) => html_escape::decode_html_entities(&value).to_string(),
        _ => String::from("")
    };
    let trailer = match document.get_by_id(Some("iframe-trailer".to_owned())) {
        Some(value) => value,
        None => return Err(Error("Error: could not get iframe-trailer".to_owned()))?
    };
    let trailer_url = match trailer.attribute(&"data-src".to_owned()) {
        Some(value) => value,
        None => return Err(Error("Error: could not get data-src".to_owned()))?
    };
    let mut released = String::new();
    let mut genres = Vec::new();
    let mut casts = Vec::new();
    let mut duration = std::time::Duration::from_secs(0);
    let mut countries = Vec::new();
    let mut producers = Vec::new();
    let element = match document.get_by_class(&"elements".to_owned()) {
        Some(value) => value,
        None => return Err(Error("Error: could not get elements".to_owned()))?
    };
    let elements = match element.get_by_class(&"row".to_owned()) {
        Some(value) => value,
        None => return Err(Error("Error: could not get row elements".to_owned()))?
    }.get_all_by_class(&"row-line".to_owned());

    for element in &elements {
        let strong = match element.get_by_name(&"strong".to_owned()) {
            Some(value) => value,
            None => return Err(Error("Error: could not get strong".to_owned()))?
        };
        let name = match strong.get_text() {
            Some(value) => value,
            None => return Err(Error("Error: could not get text for strong".to_owned()))?
        };

        if name == "Released:".to_owned() {
            released = match element.get_text() {
                Some(value) => value,
                None => return Err(Error("Error: could not get released".to_owned()))?
            };
        } else if name == "Genre:".to_owned() {
            let values = element.get_all_by_name(&"a".to_owned());
            for element in &values {
                genres.push(html_escape::decode_html_entities(match &element.attribute(&"title".to_owned()) {
                    Some(value) => value,
                    None => return Err(Error("Error: could not get genres".to_owned()))?
                }).to_string());
            }
        } else if name == "Casts:".to_owned() {
            let values = element.get_all_by_name(&"a".to_owned());
            for element in &values {
                casts.push(html_escape::decode_html_entities(match &element.attribute(&"title".to_owned()) {
                    Some(value) => value,
                    None => return Err(Error("Error: could not get casts".to_owned()))?
                }).to_string());
            }
        } else if name == "Duration:".to_owned() {
            let text = match element.get_text() {
                Some(value) => value,
                None => return Err(Error("Error: could not get duration".to_owned()))?
            };
            let string = text.split("\n").collect::<Vec<&str>>()[0];
            let int: u64 = match string.parse() {
                Ok(value) => value,
                _ => 0
            };
            duration = std::time::Duration::from_secs(int * 60);
        } else if name == "Country:".to_owned() {
            let values = element.get_all_by_name(&"a".to_owned());
            for element in &values {
                countries.push(html_escape::decode_html_entities(match &element.attribute(&"title".to_owned()) {
                    Some(value) => value,
                    None => return Err(Error("Error: could not get country".to_owned()))?
                }).to_string());
            }
        } else if name == "Production:".to_owned() {
            let values = element.get_all_by_name(&"a".to_owned());
            for element in &values {
                producers.push(html_escape::decode_html_entities(match &element.attribute(&"title".to_owned()) {
                    Some(value) => value,
                    None => return Err(Error("Error: could not get producers".to_owned()))?
                }).to_string());
            }
        }
    }

    Ok((description, trailer_url, released, genres, casts, duration, countries, producers))
}
