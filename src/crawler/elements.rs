pub fn get_servers(html: &String) -> Vec<(String, String)> {
    let documents = Document::new(html).get_all_by_name(&"a".to_owned());
    let mut servers: Vec<(String, String)> = Vec::new();

    for document in documents {
        let code = document.child_attribute(&"data-id".to_owned()).unwrap();
        let name = document.get_by_name(&"span".to_owned()).unwrap().get_text().unwrap();
        servers.push((name, code));
    }

    servers.into_iter().rev().collect::<Vec<(String, String)>>()
}

pub fn get_seasons(html: &String) -> Vec<(String, String)> {
    let documents = Document::new(html).get_all_by_name(&"a".to_owned());
    let mut seasons: Vec<(String, String)> = Vec::new();

    for document in documents {
        let code = document.attribute(&"data-id".to_owned()).unwrap();
        let name = document.get_by_name(&"a".to_owned()).unwrap().get_text().unwrap();
        seasons.push((name, code));
    }

    seasons.into_iter().rev().collect::<Vec<(String, String)>>()
}

pub fn get_episodes(html: &String) -> Vec<(i32, String, String, String)> {
    let documents = Document::new(html).get_all_by_class(&"swiper-slide".to_owned());

    let mut episodes: Vec<(i32, String, String, String)> = Vec::new();

    for document in documents {
        let image_url = document.get_by_name(&"img".to_owned()).unwrap().attribute(&"src".to_owned()).unwrap();
        let code = document.child_attribute(&"data-id".to_owned()).unwrap();
        let episode_number = document.get_by_class(&"episode-number".to_owned()).unwrap().get_text().unwrap().replace(":", "").replace("Episode ", "").replace("\n", "").parse::<i32>().unwrap();
        let name = document.get_by_class(&"film-name".to_owned()).unwrap().child_attribute(&"title".to_owned()).unwrap();
        episodes.push((episode_number, name, code, image_url));
    }

    episodes.into_iter().rev().collect::<Vec<(i32, String, String, String)>>()
}