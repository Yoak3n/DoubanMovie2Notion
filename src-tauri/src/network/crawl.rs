use crate::model::movie::{DoubanMovie, MovieQuery};
use reqwest::{Client, Url};
use serde::Deserialize;
use std::fmt;

const SUBJECT_PREFIX: &str = "https://movie.douban.com/subject/";
const QUERY_PREFIX: &str = "https://movie.douban.com/j/subject_suggest";
const IYUNS_DBYS_API: &str = "https://api.iyuns.com/api/dbys";

#[derive(Debug)]
pub enum CrawlError {
    InvalidTarget,
    Request(reqwest::Error),
    BadStatus(u16),
    CaptchaDetected,
    ApiError(String),
}

impl fmt::Display for CrawlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTarget => write!(f, "invalid target"),
            Self::Request(e) => write!(f, "request error: {e}"),
            Self::BadStatus(code) => write!(f, "douban returned status code: {code}"),
            Self::CaptchaDetected => write!(f, "detected captcha or anti-crawl page"),
            Self::ApiError(msg) => write!(f, "third-party api error: {msg}"),
        }
    }
}

impl std::error::Error for CrawlError {}

impl From<reqwest::Error> for CrawlError {
    fn from(value: reqwest::Error) -> Self {
        Self::Request(value)
    }
}

impl From<CrawlError> for String {
    fn from(value: CrawlError) -> Self {
        value.to_string()
    }
}

pub async fn crawl_douban_movie(target: &str) -> Result<DoubanMovie, CrawlError> {
    let c = Crawl::new(target)?;
    c.fetch_movie().await
}

pub async fn crawl_movie_query(name: &str) -> Result<Vec<MovieQuery>, CrawlError> {
    let mut url = Url::parse(QUERY_PREFIX).map_err(|_| CrawlError::InvalidTarget)?;
    url.query_pairs_mut().append_pair("q", name);
    let client = Client::new();
    let resp = client.get(url).send().await?;
    let status = resp.status();
    if !status.is_success() {
        return Err(CrawlError::BadStatus(status.as_u16()));
    }
    let body = resp.text().await?;
    serde_json::from_str(&body).map_err(|_| CrawlError::InvalidTarget)
}


pub struct Crawl {
    target: String,
}

impl Crawl {
    pub fn new(target: &str) -> Result<Self, CrawlError> {
        let target = target.trim().trim_matches('`').trim();
        if target.starts_with(SUBJECT_PREFIX) {
            let mut url = target.to_string();
            if !url.ends_with('/') {
                url.push('/');
            }
            return Ok(Self { target: url });
        }

        let id = target;
        if !id.is_empty() && id.chars().all(|c| c.is_ascii_digit()) {
            return Ok(Self {
                target: format!("{SUBJECT_PREFIX}{id}/"),
            });
        }

        Err(CrawlError::InvalidTarget)
    }

    pub async fn fetch_movie(&self) -> Result<DoubanMovie, CrawlError> {
        let mut api = Url::parse(IYUNS_DBYS_API).map_err(|_| CrawlError::InvalidTarget)?;
        api.query_pairs_mut().append_pair("url", &self.target);

        let client = Client::new();
        let resp = client.get(api).send().await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(CrawlError::BadStatus(status.as_u16()));
        }

        let result = resp.json::<IyunsDbysResponse>().await?;
        if !result.success {
            let primary = result
                .error
                .clone()
                .unwrap_or_else(|| "unknown error".to_string());
            return Err(CrawlError::ApiError(primary));
        }

        if let Some(requested_sid) = extract_subject_id_from_url(&self.target) {
            if let Some(sid) = result.sid.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
                if sid != requested_sid {
                    return Err(CrawlError::ApiError(format!(
                        "api returned mismatched subject id: requested {requested_sid}, got {sid}"
                    )));
                }
            }

            if let Some(format) = result.format.as_deref() {
                if let Some(format_sid) = extract_subject_id_from_format(format) {
                    if !format_sid.is_empty() && format_sid != requested_sid {
                        return Err(CrawlError::ApiError(format!(
                            "api returned mismatched subject id: requested {requested_sid}, got {format_sid}"
                        )));
                    }
                }
            }
        }

        let movie = map_iyuns_to_movie(result);
        if movie.name.trim().is_empty() {
            return Err(CrawlError::ApiError("api returned empty movie name".to_string()));
        }
        Ok(movie)
    }
}

#[derive(Deserialize)]
struct IyunsDbysResponse {
    success: bool,
    error: Option<String>,
    format: Option<String>,
    sid: Option<String>,
    imdb_id: Option<String>,
    imdb_link: Option<String>,
    chinese_title: Option<String>,
    foreign_title: Option<String>,
    this_title: Option<Vec<String>>,
    year: Option<String>,
    region: Option<Vec<String>>,
    genre: Option<Vec<String>>,
    language: Option<Vec<String>>,
    playdate: Option<Vec<String>>,
    duration: Option<String>,
    poster: Option<String>,
    director: Option<Vec<IyunsPerson>>,
    writer: Option<Vec<IyunsPerson>>,
    cast: Option<Vec<IyunsPerson>>,
    douban_rating_average: Option<String>,
}

#[derive(Deserialize)]
struct IyunsPerson {
    name: String,
}

fn map_iyuns_to_movie(resp: IyunsDbysResponse) -> DoubanMovie {
    let name = resp
        .chinese_title
        .clone()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| {
            resp.this_title
                .as_ref()
                .and_then(|v| v.first())
                .map(|s| s.to_string())
        })
        .or_else(|| resp.foreign_title.clone())
        .unwrap_or_default()
        .trim()
        .to_string();

    let director = take5_people(resp.director);
    let actor = take5_people(resp.cast);
    let writer = take5_people(resp.writer);

    let release_time = resp
        .playdate
        .as_ref()
        .and_then(|v| v.first())
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    let score = resp
        .douban_rating_average
        .as_deref()
        .and_then(|s| s.trim().parse::<f32>().ok())
        .unwrap_or(0.0);

    let genre = take5_strings(resp.genre);
    let region = take5_strings(resp.region);
    let language = take5_strings(resp.language);

    let duration = resp
        .duration
        .as_deref()
        .and_then(parse_first_int)
        .unwrap_or(0);

    let year = resp
        .year
        .as_deref()
        .and_then(parse_first_int)
        .map(|v| v as u32)
        .unwrap_or(0);

    let imdb = resp
        .imdb_id
        .clone()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| resp.imdb_link.clone())
        .unwrap_or_default()
        .trim()
        .to_string();

    let src = resp
        .poster
        .unwrap_or_default()
        .trim()
        .trim_matches('`')
        .trim()
        .to_string();

    DoubanMovie {
        name,
        director,
        actor,
        release_time,
        writer,
        score,
        genre,
        region,
        duration,
        year,
        imdb,
        language,
        rank_no: String::new(),
        src,
    }
}

fn take5_people(list: Option<Vec<IyunsPerson>>) -> Vec<String> {
    list.unwrap_or_default()
        .into_iter()
        .map(|p| normalize_person_name(&p.name))
        .filter(|s| !s.is_empty())
        .take(5)
        .collect()
}

fn normalize_person_name(input: &str) -> String {
    let raw = decode_basic_html_entities(input).trim().to_string();
    if raw.is_empty() {
        return raw;
    }

    let has_cjk = raw.chars().any(is_cjk_char);
    let has_latin = raw.chars().any(|c| is_latin_letter(c));
    if !(has_cjk && has_latin) {
        return raw;
    }

    let chinese: String = raw
        .chars()
        .filter(|&c| is_cjk_char(c) || matches!(c, '·' | '・' | '•'))
        .collect();

    let chinese = chinese.trim();
    if chinese.is_empty() {
        raw
    } else {
        chinese.to_string()
    }
}

fn decode_basic_html_entities(input: &str) -> String {
    input
        .replace("&#39;", "'")
        .replace("&quot;", "\"")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
}

fn is_latin_letter(c: char) -> bool {
    c.is_alphabetic() && !is_cjk_char(c)
}

fn is_cjk_char(c: char) -> bool {
    matches!(c, '\u{3400}'..='\u{4DBF}' | '\u{4E00}'..='\u{9FFF}' | '\u{F900}'..='\u{FAFF}')
}

fn take5_strings(list: Option<Vec<String>>) -> Vec<String> {
    list.unwrap_or_default()
        .into_iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .take(5)
        .collect()
}

fn parse_first_int(s: &str) -> Option<i32> {
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse::<i32>().ok()
    }
}

fn extract_subject_id_from_url(url: &str) -> Option<String> {
    let url = url.trim().trim_matches('`').trim();
    let s = url.strip_prefix(SUBJECT_PREFIX)?;
    let s = s.strip_suffix('/')?;
    if s.is_empty() || !s.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(s.to_string())
}

fn extract_subject_id_from_format(s: &str) -> Option<String> {
    const MARK: &str = "movie.douban.com/subject/";
    let idx = s.find(MARK)?;
    let tail = &s[idx + MARK.len()..];
    let digits: String = tail.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        Some(digits)
    }
}

#[cfg(test)]
mod tests {
    use super::Crawl;

    #[test]
    fn normalize_target_from_id() {
        let c = Crawl::new("1292052").unwrap();
        assert_eq!(c.target, "https://movie.douban.com/subject/1292052/");
    }

    #[test]
    fn normalize_target_from_url() {
        let c = Crawl::new("https://movie.douban.com/subject/1292052").unwrap();
        assert_eq!(c.target, "https://movie.douban.com/subject/1292052/");
    }
}
