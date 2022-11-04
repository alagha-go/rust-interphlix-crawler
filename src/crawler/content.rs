pub fn get_movie_content(html: &String) -> (String, String, String, Vec<String>, Vec<String>, std::time::Duration, Vec<String>, Vec<String>) {
    let document = Document::new(html);
    let description = document
        .get_by_class(&"description".to_owned())
        .unwrap()
        .get_text()
        .unwrap();
    let trailer_url = document.get_by_id(Some("iframe-trailer".to_owned())).unwrap().attribute(&"data-src".to_owned()).unwrap();
    let mut released = String::new();
    let mut genres = Vec::new();
    let mut casts = Vec::new();
    let mut duration = std::time::Duration::from_secs(0);
    let mut countries = Vec::new();
    let mut producers = Vec::new();
    let elements = document
        .get_by_class(&"elements".to_owned())
        .unwrap()
        .get_by_class(&"row".to_owned())
        .unwrap()
        .get_all_by_class(&"row-line".to_owned());
    for element in &elements {
        let name = element
            .get_by_name(&"strong".to_owned())
            .unwrap()
            .get_text()
            .unwrap();

        if name == "Released:".to_owned() {
            released = element.get_text().unwrap();
        } else if name == "Genre:".to_owned() {
            let values = element.get_all_by_name(&"a".to_owned());
            for element in &values {
                genres.push(element.attribute(&"title".to_owned()).unwrap());
            }
        } else if name == "Casts:".to_owned() {
            let values = element.get_all_by_name(&"a".to_owned());
            for element in &values {
                casts.push(element.attribute(&"title".to_owned()).unwrap());
            }
        } else if name == "Duration:".to_owned() {
            let text = element.get_text().unwrap();
            let string = text.split("\n").collect::<Vec<&str>>()[0];
            let int: u64 = string.parse().unwrap();
            duration = std::time::Duration::from_secs(int * 60);
        } else if name == "Country:".to_owned() {
            let values = element.get_all_by_name(&"a".to_owned());
            for element in &values {
                countries.push(element.attribute(&"title".to_owned()).unwrap());
            }
        } else if name == "Production:".to_owned() {
            let values = element.get_all_by_name(&"a".to_owned());
            for element in &values {
                producers.push(element.attribute(&"title".to_owned()).unwrap());
            }
        }
    }

    (description, trailer_url, released, genres, casts, duration, countries, producers)
}
