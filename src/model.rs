use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Member {
    id: String,
    name: String,
    short_info: String,
    long_info: String,
    pic: PathBuf,
}
