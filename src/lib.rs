use gon::{GonError, GonErrorReporter};
use parol_runtime::Report;
use serde_json::Value;

pub const CONTENT_DIR: &str = "content";
pub const MEMBERS_FILE: &str = "members.gon";
pub const BOOKS_FILE: &str = "books.gon";
pub const GLOBAL_FILE: &str = "global.gon";
pub const HEADER_FILE: &str = "header.html";
pub const FOOTER_FILE: &str = "footer.html";

pub const DATE_FORMAT: &str = "%d.%m.%Y %H:%M";
pub const KEY_BUILD_TIME: &str = "build_time";
pub const KEY_HEADER: &str = "header";
pub const KEY_FOOTER: &str = "footer";
pub const KEY_COMMENTS: &str = "comments";
pub const KEY_RATING: &str = "rating";
pub const KEY_FROM: &str = "from";
pub const KEY_AVERAGE_RATING: &str = "average-rating";
pub const KEY_RATING_PERCENT: &str = "rating-percent";

pub fn read_gon_object(content_dir: &std::path::PathBuf, path: &str) -> Result<Value, String> {
    let src = std::fs::read_to_string(content_dir.join(path))
        .map_err(|_| format!("missing '{dir}/{path}'!", dir = content_dir.display()))?;

    match gon::parse_file(&src, path) {
        Ok(g) => Ok(g.into()),
        Err(GonError::ParserErr(e)) => {
            GonErrorReporter::report_error(&e, path).map_err(|e| e.to_string())?;
            Err(e.to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}
