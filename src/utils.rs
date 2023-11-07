use strsim::jaro_winkler;
use std::path::{Path, PathBuf};

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

fn file_name_from_path(path: &str) -> String {
    let path = Path::new(path);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    file_name.to_string()
}

pub fn similarity_sort(vector: &mut [String], input: &str) {
    vector.sort_by(|a, b| {
        let input = input.to_lowercase();
        let a = file_name_from_path(a).to_lowercase();
        let b = file_name_from_path(b).to_lowercase();
        let a = jaro_winkler(a.as_str(), input.as_str());
        let b = jaro_winkler(b.as_str(), input.as_str());
        b.partial_cmp(&a).unwrap()
    });
}