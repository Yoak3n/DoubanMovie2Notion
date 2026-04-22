use std::env;
use std::process::Command;
#[cfg(windows)]
#[tokio::main]
async fn main() {
    let is_cli = db2n_lib::cli::run().await;
    if is_cli {
        return;
    }

    let mut exe_path = env::current_exe().expect("Failed to get current exe path");
    exe_path.set_file_name("db2n.exe");

    Command::new(&exe_path).spawn().unwrap_or_else(|e| {
        eprintln!("Failed to start db2n.exe at {:?}: {}", exe_path, e);
        std::process::exit(1);
    });
}

#[cfg(not(windows))]
fn main() {}
