use std::env;
use std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;

pub fn find<P: AsRef<Path>>(exe_name: P) -> Option<PathBuf> {
    // Get system's PATH environment variable
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe_name);
                
                if let Ok(metadata) = full_path.metadata() {
                    let is_file = metadata.is_file();

                    let is_executable = (metadata.permissions().mode() & 0o111) != 0;

                    if is_file && is_executable {
                        return Some(full_path);
                    }
                }
                None
            }).next()
    })
}
