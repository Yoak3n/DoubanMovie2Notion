use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoubanMovie {
    pub name: String,
    pub director: Vec<String>,
    pub actor: Vec<String>,
    pub release_time: String,
    pub writer: Vec<String>,
    pub score: f32,
    pub genre: Vec<String>,
    pub region: Vec<String>,
    pub duration: i32,
    pub year: u32,
    pub imdb: String,
    pub language: Vec<String>,
    pub rank_no: String,
    pub src: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieQuery {
    pub episode: String,
    pub id: String,
    pub img: String,
    pub title: String,
    pub sub_title: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub url: String,
    pub year: String,
}