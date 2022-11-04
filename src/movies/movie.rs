impl Movie {
    pub async fn crawl(tuple: (String, String, String, String), movie_type: MovieType) -> Self {
        let (name, page_url, code, image_url) = tuple;
        let html = crawler::get_request(page_url.clone()).await.unwrap();
        let (description, trailer_url, released, genres, casts, duration, countries, producers) = crawler::get_movie_content(&html);
        let id = ObjectId::new();

        Self{id, name, page_url, code, image_url, description, trailer_url, duration, genres, casts, released, movie_type, countries, producers}
    }

    pub fn default() -> Self {
        let id = ObjectId::new();
        Movie{id, ..Default::default()}
    }

    pub fn save(&self) {
        let json = serde_json::to_string(self).unwrap();
        let bytes = json.as_bytes();
        let path = format!("./DB/movies/{}.json", self.id.hex());
        let file = match File::create(&path) {
            Ok(file) => file,
            Err(err) => {
                match err.kind() {
                    std::io::ErrorKind::NotFound => {
                        fs::create_dir_all("./DB/movies").unwrap();
                        File::create(&path).unwrap()
                    },
                    _ => {
                        panic!("{}", err);
                    }
                }
            }
        };
        let mut file = BufWriter::new(file);
        file.write_all(bytes).expect("Unable to write data");
    }
}