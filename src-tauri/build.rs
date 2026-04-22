fn main() {
    #[cfg(target_os = "windows")]
    {
        // 第一次编译时，cli 还不存在，Tauri的resources引入需要它存在。
        // 因此，我们需要在构建时创建一个空的占位文件，来欺骗 Tauri。
        // Tauri 的构建脚本会尝试检查资源文件是否存在，如果不存在则会报错。
        // 我们通过创建一个空的占位文件来欺骗 Tauri，使其能通过预检查。
        // 随后 Cargo 编译真正的 entry.exe 时，会覆盖这个占位文件。
        use std::fs;
        use std::path::Path;

        let target_dir = Path::new("target/release");
        if !target_dir.exists() {
            fs::create_dir_all(target_dir).unwrap_or_default();
        }
        
        let entry_path = target_dir.join("db2n-cli.exe");
        if !entry_path.exists() {
            fs::write(entry_path, "").unwrap_or_default();
        }
    }
    tauri_build::build()
}
