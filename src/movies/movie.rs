impl Movie {
    pub async fn crawl(tuple: (String, String, String, String), movie_type: MovieType) -> Result<Self> {
        let (name, page_url, code, image_url) = tuple;
        if Movie::exists(&code) {
            return Ok(Movie::default()?)
        }
        let html = crawler::get_request(page_url.clone()).await?;
        let (description, trailer_url, released, genres, casts, duration, countries, producers) =
            crawler::get_movie_content(&html)?;
        let id = ObjectId::new()?;

        let (seasons, servers) = match movie_type {
            MovieType::Movie => {
                let seasons: Vec<Season> = Vec::new();
                let url = format!("{}{code}", &*crate::MOVIESERVERSURL);
                let servers = collect_servers(&url).await?;

                (seasons, servers)
            }
            MovieType::Tvshow => {
                let servers: Vec<Server> = Vec::new();
                let url = format!("{}{code}", &*crate::SEASONSURL);
                let seasons = collect_seasons(&url).await?;

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
        movie.save()?;
        crate::CODES.write().insert(movie.code.clone(), movie.id.id);
        Ok(movie)
    }

    pub fn default() -> Result<Self> {
        let id = ObjectId::new()?;
        Ok(Movie {
            id,
            ..Default::default()
        })
    }

    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string(self)?;
        let bytes = json.as_bytes();
        let path = format!("{}{}.json", &*crate::MOVIESPATH, self.id.hex());
        let file = match File::create(&path) {
            Ok(file) => file,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    fs::create_dir_all(&*crate::MOVIESPATH)?;
                    File::create(&path)?
                }
                _ => return Err(err)?
            },
        };
        let mut writer = BufWriter::new(file);
        writer.write_all(bytes).expect("Unable to write data");

        Ok(())
    }

    pub fn exists(code: &String) -> bool {
        crate::CODES.read().get(code) != None
    }

    pub fn save_codes() -> Result<()> {
        let data = crate::CODES.read();
        let data = data.deref();
        let json = serde_json::to_string(data)?;
        let bytes = json.as_bytes();
        let path = format!("{}exists.json", &*crate::MOVIESPATH);
        let file = File::create(&path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(bytes)?;

        Ok(())
    }

    pub fn codes() -> Result<HashMap<String, [u8; 12]>> {
        let path = format!("{}exists.json", &*crate::MOVIESPATH);
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    fs::create_dir_all(&*crate::MOVIESPATH)?;
                    File::create(&path)?;
                    let codes: HashMap<String, [u8; 12]> = HashMap::new();
                    return Ok(codes)
                },
                _ => return Err(err)?
            }
        };

        let mut buf: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(file);

        let _ = reader.read_to_end(&mut buf);
        let json = String::from_utf8(buf)?;
        
        Ok(serde_json::from_str(&json)?)
    }
}