use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct FileItem {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<String>,
}

/// Reads a directory and returns a sorted vector of FileItems (folders first, then files).
pub fn read_dir(path: &Path) -> std::io::Result<Vec<FileItem>> {
    let mut items = Vec::new();

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let path_buf = entry.path();
            
            let name = path_buf
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();

            let modified = metadata.modified().ok().map(format_time);

            items.push(FileItem {
                name,
                path: path_buf,
                is_dir: metadata.is_dir(),
                size: metadata.len(),
                modified,
            });
        }
    }

    // Sort: Directories first alphabetically, then files alphabetically
    items.sort_by(|a, b| {
        b.is_dir.cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    Ok(items)
}

fn format_time(systime: SystemTime) -> String {
    let datetime: DateTime<Local> = systime.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}