use std::env;

#[cfg(target_os = "windows")]
pub fn add_to_path(install_dir: &str) -> Result<(), String> {
    let (current_path, value_type) = read_user_path()?;
    let mut entries = split_path_entries(&current_path);
    if entries
        .iter()
        .any(|entry| same_path(entry, install_dir))
    {
        return Ok(());
    }

    entries.push(install_dir.to_string());
    write_user_path(&entries.join(";"), value_type)
}

#[cfg(target_os = "windows")]
pub fn remove_from_path(install_dir: &str) -> Result<(), String> {
    let (current_path, value_type) = read_user_path()?;
    let entries = split_path_entries(&current_path);
    let filtered: Vec<String> = entries
        .into_iter()
        .filter(|entry| !same_path(entry, install_dir))
        .collect();

    write_user_path(&filtered.join(";"), value_type)
}

#[cfg(target_os = "windows")]
fn get_current_path() -> Result<String, String> {
    read_user_path().map(|(path, _)| path)
}

#[cfg(target_os = "windows")]
pub fn is_in_path(install_dir: &str) -> bool {
    if let Ok(current_path) = get_current_path() {
        current_path
            .split(';')
            .filter(|entry| !entry.is_empty())
            .any(|entry| same_path(entry, install_dir))
    } else {
        false
    }
}

#[cfg(target_os = "windows")]
fn split_path_entries(path: &str) -> Vec<String> {
    path.split(';')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(|entry| entry.to_string())
        .collect()
}

#[cfg(target_os = "windows")]
fn same_path(left: &str, right: &str) -> bool {
    normalize_path(left).eq_ignore_ascii_case(&normalize_path(right))
}

#[cfg(target_os = "windows")]
fn normalize_path(path: &str) -> String {
    path.trim()
        .trim_end_matches(['\\', '/'])
        .to_string()
}

#[cfg(target_os = "windows")]
fn read_user_path() -> Result<(String, winreg::enums::RegType), String> {
    use std::io::ErrorKind;
    use winreg::{
        enums::{HKEY_CURRENT_USER, KEY_QUERY_VALUE, KEY_SET_VALUE, REG_EXPAND_SZ, REG_SZ},
        RegKey,
    };

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env_key = hkcu
        .open_subkey_with_flags("Environment", KEY_QUERY_VALUE | KEY_SET_VALUE)
        .map_err(|e| e.to_string())?;

    match env_key.get_raw_value("Path") {
        Ok(value) => {
            let value_type = match value.vtype {
                REG_EXPAND_SZ => REG_EXPAND_SZ,
                _ => REG_SZ,
            };
            decode_reg_string(&value).map(|path| (path, value_type))
        }
        Err(error) if error.kind() == ErrorKind::NotFound => Ok((String::new(), REG_EXPAND_SZ)),
        Err(error) => Err(error.to_string()),
    }
}

#[cfg(target_os = "windows")]
fn write_user_path(path: &str, value_type: winreg::enums::RegType) -> Result<(), String> {
    use winreg::{
        enums::{HKEY_CURRENT_USER, KEY_SET_VALUE},
        RegKey, RegValue,
    };

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env_key = hkcu
        .open_subkey_with_flags("Environment", KEY_SET_VALUE)
        .map_err(|e| e.to_string())?;

    let value = RegValue {
        vtype: value_type,
        bytes: encode_reg_string(path),
    };

    env_key
        .set_raw_value("Path", &value)
        .map_err(|e| e.to_string())?;
    env::set_var("PATH", path);
    Ok(())
}

#[cfg(target_os = "windows")]
fn decode_reg_string(value: &winreg::RegValue) -> Result<String, String> {
    let mut units = Vec::with_capacity(value.bytes.len() / 2);
    for chunk in value.bytes.chunks_exact(2) {
        units.push(u16::from_le_bytes([chunk[0], chunk[1]]));
    }
    while units.last() == Some(&0) {
        units.pop();
    }
    String::from_utf16(&units).map_err(|e| e.to_string())
}

#[cfg(target_os = "windows")]
fn encode_reg_string(value: &str) -> Vec<u8> {
    let mut wide: Vec<u16> = value.encode_utf16().collect();
    wide.push(0);
    wide.into_iter()
        .flat_map(|unit| unit.to_le_bytes())
        .collect()
}

#[cfg(target_os = "windows")]
pub fn get_install_dir_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        if arg == "--install-dir" && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
    }
    None
}
