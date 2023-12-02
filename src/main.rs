
use std::{path::{Path, PathBuf}, collections::HashSet};

/// Returns a PathBuf to the target volume, if it exists.
/// 
/// # Arguments
/// 
/// * `path` - A string slice of the path to the target volume.
fn target_volume(path: &str) -> Option<PathBuf> {
    let path = Path::new(path);
    if path.exists() {
        Some(path.to_path_buf())
    } else {
        None
    }
}

/// Returns an iterator over the recordings in the given directory.
/// Skips any hidden files/folders.
/// 
/// # Arguments
/// 
/// * `dir` - A PathBuf reference to the directory to search for recordings in.
/// 
fn recordings(dir: &PathBuf) -> impl Iterator<Item = walkdir::DirEntry> {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_entry( |e|
            e.file_name()
                .to_str()
                .map(|s| !s.starts_with(".")) // Exclude hidden files/folders
                .unwrap_or(false),
        )
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "mp3"))
}


fn main() {
    let mut prev_recordings = HashSet::<PathBuf>::new();

    loop {
        match target_volume("/Volumes/IC RECORDER") {
            Some(path) => {                
                let current_recordings: HashSet<PathBuf> = recordings(&path).map(|e| e.path().to_path_buf()).collect(); 
                let new_recordings = current_recordings.difference(&prev_recordings);
                
                for entry in new_recordings {
                    println!("{}", entry.display());
                }

                prev_recordings = current_recordings;
            }
            None => {}
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
