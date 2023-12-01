fn main() {
    println!("Hello, world! Here are the drives you wanted.");



  /*   let paths = std::fs::read_dir("/Volumes/IC RECORDER").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() && !path.to_string_lossy().starts_with(".") {
            println!("{}", path.display());
            let files = std::fs::read_dir(path).unwrap();
            for file in files {
                let file = file.unwrap().path();
                if file.is_dir() && !file.to_string_lossy().starts_with(".") {
                    println!("  - {}", file.display());
                    let sub_files = std::fs::read_dir(file).unwrap();
                    for sub_file in sub_files {
                        let sub_file = sub_file.unwrap().path();
                        if !sub_file.to_string_lossy().starts_with(".") {
                            println!("     - {}", sub_file.display());
                        }
                    }
                }
            }
        }
    } */

    use std::fs;
    use std::time::Duration;
    use std::thread;
    use std::collections::HashSet;

    let mut drives_before = HashSet::new();

    loop {
        let mut drives_after = HashSet::new();
        for entry in fs::read_dir("/Volumes").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                drives_after.insert(path);
            }
        }

        if drives_before != drives_after {
            for drive in &drives_after {
                if !drives_before.contains(drive) {
                    println!("Drive mounted: {}", drive.display());
                }
            }
            for drive in &drives_before {
                if !drives_after.contains(drive) {
                    println!("Drive unmounted: {}", drive.display());
                }
            }
        }

        drives_before = drives_after;

        thread::sleep(Duration::from_secs(1));
    }
}
