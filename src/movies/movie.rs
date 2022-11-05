impl Movie {
    pub async fn crawl(tuple: (String, String, String, String), movie_type: MovieType) -> Self {
        let (name, page_url, code, image_url) = tuple;
        if Movie::exists(&code) {
            return Movie::default()
        }
        let html = crawler::get_request(page_url.clone()).await.unwrap();
        let (description, trailer_url, released, genres, casts, duration, countries, producers) =
            crawler::get_movie_content(&html);
        let id = ObjectId::new();

        let (seasons, servers) = match movie_type {
            MovieType::Movie => {
                let seasons: Vec<Season> = Vec::new();
                let url = format!("{}{code}", &*crate::MOVIESERVERSURL);
                let servers = collect_servers(&url).await;

                (seasons, servers)
            }
            MovieType::Tvshow => {
                let servers: Vec<Server> = Vec::new();
                let url = format!("{}{code}", &*crate::SEASONSURL);
                let seasons = collect_seasons(&url).await;

                (seasons, servers)
            }
        };

        let movie = Self {
            id,
            name,
            page_url,
            code,
            image_url,
            description,
            trailer_url,
            duration,
            genres,
            casts,
            released,
            movie_type,
            countries,
            producers,
            seasons,
            servers,
        };
        movie.save();
        crate::CODES.write().push(movie.code.clone());
        movie
    }

    pub fn default() -> Self {
        let id = ObjectId::new();
        Movie {
            id,
            ..Default::default()
        }
    }

    pub fn save(&self) {
        let json = serde_json::to_string(self).unwrap();
        let bytes = json.as_bytes();
        let path = format!("{}{}.json", &*crate::MOVIESPATH, self.id.hex());
        let file = match File::create(&path) {
            Ok(file) => file,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    fs::create_dir_all(&*crate::MOVIESPATH).unwrap();
                    File::create(&path).unwrap()
                }
                _ => {
                    panic!("{}", err);
                }
            },
        };
        let mut writer = BufWriter::new(file);
        writer.write_all(bytes).expect("Unable to write data");
    }

    pub fn exists(code: &String) -> bool {
        crate::CODES.read().contains(code)
    }

    pub fn save_codes() {
        let data = crate::CODES.read();
        let data = data.deref();
        let json = serde_json::to_string(data).unwrap();
        let bytes = json.as_bytes();
        let path = format!("{}exists.json", &*crate::MOVIESPATH);
        let file = File::create(&path).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write_all(bytes).expect("unable to write data");
    }

    pub fn codes() -> Vec<String> {
        let path = format!("{}exists.json", &*crate::MOVIESPATH);
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    let _ = fs::create_dir_all(&*crate::MOVIESPATH);
                    let _ = File::create(&path).unwrap();
                    let codes: Vec<String> = Vec::new();
                    return codes
                },
                _ => panic!("{}", err)
            }
        };

        let mut buf: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(file);

        let _ = reader.read_to_end(&mut buf);
        let json = String::from_utf8(buf).unwrap();
        
        serde_json::from_str(&json).unwrap()
    }
}