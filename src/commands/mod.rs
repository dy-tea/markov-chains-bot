use std::path::PathBuf;

mod messages;
mod tokens;
mod dataset;
mod model;
mod params;
mod query;

pub use messages::messages;
pub use tokens::tokens;
pub use dataset::dataset;
pub use model::model;
pub use params::params;
pub use query::query;

pub fn search_files(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let mut paths = paths.into_iter()
        .map(|path| path.into())
        .collect::<Vec<PathBuf>>();

    while let Some(path) = paths.pop() {
        if path.is_file() {
            files.push(path);
        }

        else if path.is_dir() {
            if let Ok(dir_paths) = path.read_dir() {
                let dir_paths = dir_paths.flatten()
                    .map(|path| path.path())
                    .collect::<Vec<_>>();

                paths.extend(dir_paths);
            }
        }
    }

    files
}
