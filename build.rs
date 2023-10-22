use std::env;
use std::fs;
use std::path::Path;

#[path = "src/settings/constants.rs"]
mod settings;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(settings::SETTINGS_PATH);

    // Check if the file exists in the destination directory
    if dest_path.exists() {
        // Check if the source file was modified since the last build
        let src_path = Path::new(settings::SETTINGS_PATH);
        let src_modified = fs::metadata(&src_path).unwrap().modified().unwrap();
        let dest_modified = fs::metadata(&dest_path).unwrap().modified().unwrap();
        if src_modified <= dest_modified {
            // The source file was not modified since the last build, no need to copy
            return;
        }
    }

    // Copy the file to the destination directory
    fs::copy(settings::SETTINGS_PATH, dest_path).unwrap();
}
