pub mod config;
pub mod command;
pub mod utils;
pub mod network;
pub mod model;
pub mod cli;



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            command::get_options,
            command::crawl_douban_movie,
            command::pick_option,
            command::write_config,
            command::query_movie,
            command::add_to_path,
            command::remove_from_path,
            command::is_path_added,
            command::get_install_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
