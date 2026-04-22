use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{
    env,
    ffi::OsStr,
    fs,
    io::Error,
    path::PathBuf,
    sync::{Arc, Mutex},
};

const APP_NAME: &str = "db2n";

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub notion: Arc<Mutex<NotionConfig>>,
    pub index: Arc<Mutex<String>>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            notion: Arc::new(Mutex::new(NotionConfig::default())),
            index: Arc::new(Mutex::new(String::new())),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NotionConfig {
    #[serde(rename = "database_id")]
    pub database_id: String,
    #[serde(rename = "token")]
    pub token: String,
}

impl Default for NotionConfig {
    fn default() -> Self {
        Self {
            database_id: String::new(),
            token: String::new(),
        }
    }
}

impl Configuration {
    fn new() -> Self {
        let options = match Self::load_options() {
            Ok(options) => options,
            Err(e) => {
                eprintln!("Error loading options: {:?}", e);
                return Self::default();
            }
        };
        if options.0.is_empty() {
            return Self::default();
        }
        let index = options.1;
        let index = if index.is_empty() {
            &options.0[0]
        } else {
            &index
        };
        match load_config(index) {
            Ok(c) => Self {
                notion: Arc::new(Mutex::new(c)),
                index: Arc::new(Mutex::new(index.to_string())),
            },
            Err(e) => {
                eprintln!("Error loading config: {:?}", e);
                Self::default()
            }
        }
    }

    pub fn global() -> &'static Self {
        static GLOBAL_CONFIG: OnceCell<Configuration> = OnceCell::new();
        GLOBAL_CONFIG.get_or_init(|| Self::new())
    }

    pub fn set(&self, name: &str) -> Result<(), Error> {
        let config = match load_config(name) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error loading config: {:?}", e);
                return Err(e);
            }
        };
        let mut notion = self.notion.lock().unwrap();
        *notion = config;
        let mut index = self.index.lock().unwrap();
        *index = name.to_string();
        update_config_index(name)?;
        Ok(())
    }

    pub fn get_notion(&self) -> NotionConfig {
        self.notion.lock().unwrap().clone()
    }
    
    pub fn get_index(&self) -> String {
        self.index.lock().unwrap().clone()
    }

    pub fn load_options() -> Result<(Vec<String>, String), Error> {
        let exists = ensure_config_dir()?;
        if !exists {
            return Ok((Vec::new(), String::new()));
        }
        let mut options = Vec::new();
        let mut index = String::new();
        // 读取指定路径下所有json文件，将文件内容读取到options中，
        let entry = fs::read_dir(config_dir())?;
        for entry in entry {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension() == Some(OsStr::new("json")) {
                let name = path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
                if !name.is_empty() {
                    options.push(name);
                }
            }
            if path.file_name() == Some(OsStr::new("index")) {
                index = fs::read_to_string(path)?;
                index = index.trim().to_string();
                index = index.trim_end_matches(".json").to_string();
            }
        }
        Ok((options, index))
    }
}

fn update_config_index(name: &str) -> Result<(), Error> {
    ensure_config_dir()?;
    fs::write(config_index_path(), normalize_config_name(name))?;
    Ok(())
}

fn ensure_config_dir() -> Result<bool, Error> {
    let dir = config_dir();
    let exists = fs::exists(&dir)?;
    if !exists {
        if let Err(e) = fs::create_dir_all(&dir) {
            println!("Error creating config dir: {:?}", e);
            return Err(e);
        }
    }
    Ok(exists)
}

pub fn create_config(name: &str, config: &NotionConfig) -> Result<(), Error> {
    ensure_config_dir()?;
    let content = serde_json::to_string_pretty(config)?;
    let path = config_file_path(name);
    fs::write(path, content)?;
    Ok(())
}

pub fn load_config(name: &str) -> Result<NotionConfig, Error> {
    ensure_config_dir()?;
    let path = config_file_path(name);
    let content = fs::read_to_string(path)?;
    let config = serde_json::from_str::<NotionConfig>(&content)?;
    Ok(config)
}

pub fn config_dir_path() -> PathBuf {
    config_dir()
}

fn config_dir() -> PathBuf {
    #[cfg(not(debug_assertions))]
    {
        if let Ok(exe) = env::current_exe() {
            if let Some(parent) = exe.parent() {
                return parent.join("data").join("keys");
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        if let Some(userprofile) = env::var_os("USERPROFILE") {
            return PathBuf::from(userprofile).join(".config").join(APP_NAME).join("keys");
        }
        if let Some(home) = env::var_os("HOME") {
            return PathBuf::from(home).join(".config").join(APP_NAME).join("keys");
        }
    }

    if let Some(appdata) = env::var_os("APPDATA") {
        return PathBuf::from(appdata).join(APP_NAME).join("keys");
    }
    PathBuf::from("data").join("keys")
}

fn config_index_path() -> PathBuf {
    config_dir().join("index")
}

fn normalize_config_name(name: &str) -> String {
    name.trim().trim_end_matches(".json").to_string()
}

fn config_file_path(name: &str) -> PathBuf {
    let base = normalize_config_name(name);
    config_dir().join(format!("{base}.json"))
}
