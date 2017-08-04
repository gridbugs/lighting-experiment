use std::path::{PathBuf, Path};
use std::env;

use res::files::RES_DIR;

pub fn res_dir() -> PathBuf {
    let mut exe_path = env::current_exe()
        .expect("Failed to find executable path");

    exe_path.pop();

    exe_path.join(RES_DIR)
}

pub fn res_path<P: AsRef<Path>>(path: P) -> PathBuf {
    res_dir().join(path)
}
