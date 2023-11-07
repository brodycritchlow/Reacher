pub fn convert(b: f64, pow: u32) -> u64 {
    (b * 1024_u64.pow(pow) as f64) as u64
}

pub fn replace_tilde_with_home_dir(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();
    if path.starts_with("~") {
        if let Some(home_dir) = dirs::home_dir() {
            // Remove the tilde from the path and append it to the home directory
            return home_dir.join(path.strip_prefix("~").unwrap());
        }
    }
    path.to_path_buf()
}