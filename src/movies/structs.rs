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
    pub countries: Vec<String>
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Season {

}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Episode {

}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum MovieType {
    #[default]
    Movie,
    Tvshow
}

#[derive(Default, Clone, Serialize)]
pub struct ObjectId {
    pub id:[u8; 12]
}
struct ObjectIdVisitor;