use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Data {
    #[serde(rename = "properties")]
    pub properties: Properties,
    #[serde(rename = "parent")]
    pub parent: Parent,
}

#[derive(Debug, Serialize, Clone)]
pub struct Parent {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "database_id")]
    pub database_id: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Properties {
    #[serde(rename = "Movie")]
    pub movie: Movie,
    #[serde(rename = "Director")]
    pub director: MultiSelectProperty,
    #[serde(rename = "Actor")]
    pub actor: MultiSelectProperty,
    #[serde(rename = "UpDate")]
    pub up_date: RichTextProperty,
    #[serde(rename = "Region")]
    pub region: MultiSelectProperty,
    #[serde(rename = "Writer")]
    pub writer: MultiSelectProperty,
    #[serde(rename = "Genre")]
    pub genre: MultiSelectProperty,
    #[serde(rename = "Cover")]
    pub cover: FilesProperty,
    #[serde(rename = "Language")]
    pub language: MultiSelectProperty,
    #[serde(rename = "Year")]
    pub year: NumberProperty<i32>,
    #[serde(rename = "imdb")]
    pub imdb: RichTextProperty,
    #[serde(rename = "Duration")]
    pub duration: NumberProperty<i32>,
    #[serde(rename = "Rank", skip_serializing_if = "Option::is_none")]
    pub rank: Option<NumberProperty<i64>>,
    #[serde(rename = "Score", skip_serializing_if = "Option::is_none")]
    pub score: Option<NumberProperty<f32>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Movie {
    #[serde(rename = "title")]
    pub title: Vec<RichTextItem>,
}

#[derive(Debug, Serialize, Clone)]
pub struct MultiSelectProperty {
    #[serde(rename = "multi_select")]
    pub multi_select: Vec<MultiSelectItem>,
}

#[derive(Debug, Serialize, Clone)]
pub struct MultiSelectItem {
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct RichTextProperty {
    #[serde(rename = "rich_text")]
    pub rich_text: Vec<RichTextItem>,
}

#[derive(Debug, Serialize, Clone)]
pub struct RichTextItem {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "text")]
    pub text: TextContent,
}

#[derive(Debug, Serialize, Clone)]
pub struct TextContent {
    #[serde(rename = "content")]
    pub content: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct FilesProperty {
    #[serde(rename = "files")]
    pub files: Vec<FileItem>,
}

#[derive(Debug, Serialize, Clone)]
pub struct FileItem {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "external")]
    pub external: ExternalFile,
}

#[derive(Debug, Serialize, Clone)]
pub struct ExternalFile {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct NumberProperty<T> {
    #[serde(rename = "number")]
    pub number: T,
}

use super::movie::DoubanMovie;

impl Data {
    pub fn from_douban_movie(movie: &DoubanMovie, database_id: &str) -> Self {
        let director = to_multi_select(&movie.director);
        let actor = to_multi_select(&movie.actor);
        let writer = to_multi_select(&movie.writer);
        let genre = to_multi_select(&movie.genre);
        let region = to_multi_select(&movie.region);
        let language = to_multi_select(&movie.language);

        let score = if movie.score > 0.0 {
            Some(NumberProperty { number: movie.score })
        } else {
            None
        };

        let rank = if movie.rank_no.trim().is_empty() {
            None
        } else {
            movie.rank_no.trim().parse::<i64>().ok().map(|n| NumberProperty { number: n })
        };

        Self {
            parent: Parent {
                kind: "database_id".to_string(),
                database_id: database_id.to_string(),
            },
            properties: Properties {
                movie: Movie {
                    title: vec![rich_text(&movie.name)],
                },
                director: MultiSelectProperty { multi_select: director },
                actor: MultiSelectProperty { multi_select: actor },
                up_date: RichTextProperty {
                    rich_text: vec![rich_text(&movie.release_time)],
                },
                region: MultiSelectProperty { multi_select: region },
                writer: MultiSelectProperty { multi_select: writer },
                genre: MultiSelectProperty { multi_select: genre },
                cover: FilesProperty {
                    files: if movie.src.trim().is_empty() {
                        Vec::new()
                    } else {
                        vec![FileItem {
                            name: "Cover".to_string(),
                            kind: "external".to_string(),
                            external: ExternalFile {
                                url: movie.src.clone(),
                            },
                        }]
                    },
                },
                language: MultiSelectProperty { multi_select: language },
                year: NumberProperty {
                    number: movie.year as i32,
                },
                imdb: RichTextProperty {
                    rich_text: vec![rich_text(&movie.imdb)],
                },
                duration: NumberProperty {
                    number: movie.duration,
                },
                rank,
                score,
            },
        }
    }
}

fn rich_text(content: &str) -> RichTextItem {
    RichTextItem {
        kind: "text".to_string(),
        text: TextContent {
            content: content.to_string(),
        },
    }
}

fn to_multi_select(values: &[String]) -> Vec<MultiSelectItem> {
    values
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .take(5)
        .map(|name| MultiSelectItem {
            name: name.to_string(),
        })
        .collect()
}