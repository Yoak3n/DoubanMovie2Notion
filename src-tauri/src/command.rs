use crate::{
    config::{self, Configuration, NotionConfig},
    model::{movie::MovieQuery, notion::Data},
    network,
};

fn path_feature_enabled() -> bool {
    cfg!(not(debug_assertions))
}

fn path_feature_disabled_error() -> String {
    "PATH 功能在开发模式下不可用".to_string()
}

#[tauri::command]
pub fn add_to_path(install_dir: String) -> Result<(), String> {
    if !path_feature_enabled() {
        return Err(path_feature_disabled_error());
    }
    crate::utils::add_to_path(&install_dir)
}

#[tauri::command]
pub fn remove_from_path(install_dir: String) -> Result<(), String> {
    if !path_feature_enabled() {
        return Err(path_feature_disabled_error());
    }
    crate::utils::remove_from_path(&install_dir)
}

#[tauri::command]
pub fn is_path_added(install_dir: String) -> bool {
    if !path_feature_enabled() {
        return false;
    }
    crate::utils::is_in_path(&install_dir)
}

#[tauri::command]
pub fn get_install_dir() -> Result<String, String> {
    if !path_feature_enabled() {
        return Err(path_feature_disabled_error());
    }
    std::env::current_exe()
        .map(|p| p.parent().map(|p| p.to_path_buf()))
        .ok()
        .flatten()
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| "Failed to get install directory".to_string())
}

#[tauri::command]
pub async fn crawl_douban_movie(target: &str) -> Result<String, String> {
    let movie = network::crawl::crawl_douban_movie(target).await?;
    println!("Crawled movie: {:?}", movie);

    let notion = Configuration::global().get_notion();
    let data = Data::from_douban_movie(&movie, &notion.database_id);
    if let Err(e) = network::notion::post_to_notion(&data).await {
        println!("Error posting to Notion: {:?}", e);
        return Err(e.to_string());
    }
    Ok(movie.name)
}


#[tauri::command]
pub async fn get_options() -> Result<(Vec<String>, String), String> {
    let o = match Configuration::load_options() {
        Ok(v) => {
            let mut options = v.0;
            let mut index = v.1;
            options.push("创建新配置".to_string());
            if index.is_empty(){
                index = options[0].clone(); 
            }
            (options, index)
        },
        Err(e) => {
            eprintln!("Error loading options: {:?}", e);
            return Err(e.to_string());
        }
    };
    Ok(o)
}

#[tauri::command]
pub async fn pick_option(index: &str) -> Result<NotionConfig, String> {
    if index == "创建新配置"{
        return Ok(Configuration::global().get_notion());
    }
    if let Err(e) = Configuration::global().set(index){
        return Err(e.to_string());
    }
    Ok(Configuration::global().get_notion())
}


#[tauri::command]
pub async fn write_config(config_name: &str, database_id: &str, token: &str) -> Result<String, String> {
    let config = NotionConfig {
        database_id: database_id.to_string(),
        token: token.to_string(),
    };
    
    if let Err(e) = config::create_config(config_name, &config){
        return Err(e.to_string());
    }
    Ok("success".to_string())
}

#[tauri::command]
pub async fn query_movie(name: &str) -> Result<Vec<MovieQuery>, String> {
    let queries = network::crawl::crawl_movie_query(name).await?;
    Ok(queries)
}
