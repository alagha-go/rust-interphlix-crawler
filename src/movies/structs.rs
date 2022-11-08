#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Movie {
    #[serde(serialize_with = "ObjectId::serialize")]
    pub id: ObjectId,
    pub name: String,
    pub code: String,
    pub movie_type: MovieType,
    pub description: String,
    pub genres: Vec<String>,
    pub page_url: String,
    pub image_url: String,
    pub trailer_url: String,
    pub released: String,
    pub duration: std::time::Duration,
    pub casts: Vec<String>,
    pub producers: Vec<String>,
    pub countries: Vec<String>,
    pub seasons: Vec<Season>,
    pub servers: Vec<Server>
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Season {
    #[serde(serialize_with = "ObjectId::serialize")]
    id: ObjectId,
    name: String,
    code: String,
    episodes: Vec<Episode>
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Episode {
    #[serde(serialize_with = "ObjectId::serialize")]
    id: ObjectId,
    number: i32,
    name: String,
    code: String,
    image_url: String,
    servers: Vec<Server>
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Server {
    #[serde(serialize_with = "ObjectId::serialize")]
    id: ObjectId,
    name: String,
    video_id: String
}


#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum MovieType {
    #[default]
    Movie,
    Tvshow
}