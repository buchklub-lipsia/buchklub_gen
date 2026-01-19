use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use chrono::Local;
use serde_json::{json, to_string_pretty, Map, Value};
use handlebars::Handlebars;

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

fn main() -> Result<(), String> {
    let content_dir = PathBuf::from(CONTENT_DIR);
    if !content_dir.is_dir() {
        return Err(format!("missing '{CONTENT_DIR}' sub dir!"));
    }

    let mut members_json = read_gon_object(&content_dir, MEMBERS_FILE)?;
    let members_map = members_json
        .as_object_mut()
        .ok_or_else(|| format!("{MEMBERS_FILE} has to contain an object!"))?;
    let mut books_json = read_gon_object(&content_dir, BOOKS_FILE)?;
    let books_array = books_json
        .as_array_mut()
        .ok_or_else(|| format!("{BOOKS_FILE} has to contain an array!"))?;
    let mut global_json = read_gon_object(&content_dir, GLOBAL_FILE)?;
    let global_map = global_json
        .as_object_mut()
        .ok_or_else(|| format!("{GLOBAL_FILE} has to contain an object!"))?;
    let mut root_map = Map::new();

    global_map.insert(
        String::from(KEY_BUILD_TIME),
        Value::String(Local::now().format(DATE_FORMAT).to_string()),
    );
    global_map.insert(
        String::from(KEY_HEADER),
        Value::String(read_file(&content_dir, HEADER_FILE)?),
    );
    global_map.insert(
        String::from(KEY_FOOTER),
        Value::String(read_file(&content_dir, FOOTER_FILE)?),
    );

    let ratings_by_member = extract_ratings(books_array, &members_map)?;
    for (id, member) in members_map.iter_mut() {
        let Some(ratings) = ratings_by_member.get(id) else {
            continue;
        };
        let member_obj = member
            .as_object_mut()
            .ok_or_else(|| format!("member is not an object: '{id:?}'"))?;
        let ratings_sum: f64 = ratings.iter().sum();
        member_obj.insert(
            String::from(KEY_AVERAGE_RATING),
            json!(to_string_2_dec_places(ratings_sum / ratings.len() as f64)),
        );
    }

    let members_list: Vec<_> = members_map.clone()
        .iter_mut()
        .map(|(id, member)| {
            member.as_object_mut().unwrap().insert(String::from("id"), json!(id));
            member.clone() //?
        })
    .collect();

    root_map.insert(String::from("members"), members_json);
    root_map.insert(String::from("members-list"), json!(members_list));
    root_map.insert(String::from("global"), global_json);
    root_map.insert(String::from("books"), books_json);

    // TODO \\
    let root = Value::Object(root_map);
    let mut handlebars = Handlebars::new();
    handlebars.register_partial(
        "book",
        std::fs::read_to_string(content_dir.join("book.html")).expect("book.html"),
    ).map_err(template_err_to_string)?;
    handlebars
        .register_template_file("index", "content/index.html")
        .map_err(|e| {
            println!("{e}");
            e
        }).map_err(template_err_to_string)?;
    handlebars.register_template_file("books", "content/books.html").map_err(template_err_to_string)?;
    handlebars.register_template_file("members", "content/members.html").map_err(template_err_to_string)?;
    handlebars.register_template_file("statute", "content/statute.html").map_err(template_err_to_string)?;
    handlebars.register_template_file("contact", "content/contact.html").map_err(template_err_to_string)?;
    handlebars.register_template_file("misc", "content/misc.html").map_err(template_err_to_string)?;

    let dst_dir = PathBuf::from("../buchklub");
    // create index.html
    render_file(&dst_dir, &handlebars, &root, "index", "index.html");
    // create books.html
    render_file(&dst_dir, &handlebars, &root, "books", "books.html");
    // create members.html
    render_file(&dst_dir, &handlebars, &root, "members", "members.html");
    // create statute.html
    render_file(&dst_dir, &handlebars, &root, "statute", "statute.html");
    render_file(&dst_dir, &handlebars, &root, "contact", "contact.html");
    render_file(&dst_dir, &handlebars, &root, "misc", "misc.html");
    // copy global.css
    std::fs::copy(content_dir.join("global.css"), dst_dir.join("global.css"))
        .expect("copying global.css");
    // copy global.js
    std::fs::copy(content_dir.join("global.js"), dst_dir.join("global.js"))
        .expect("copying global.js");
    // copy sorttable.js
    std::fs::copy(content_dir.join("sorttable.js"), dst_dir.join("sorttable.js"))
        .expect("copying sorttable.js");
    // copy collapser.js
    std::fs::copy(content_dir.join("collapser.js"), dst_dir.join("collapser.js"))
        .expect("copying collapser.js");
    // copy img/
    cp_dir(content_dir.join("img"), dst_dir.join("img")).expect("copying img/");
    // TODO-END \\

    Ok(())
}

fn template_err_to_string(e: handlebars::TemplateError) -> String {
    e.to_string()
}

fn extract_ratings(
    books_array: &mut [Value],
    members_map: &Map<String, Value>,
) -> Result<HashMap<String, Vec<f64>>, String> {
    let mut member_ratings = HashMap::new();
    for book in books_array {
        let book = book
            .as_object_mut()
            .ok_or_else(|| format!("{BOOKS_FILE} has to contain an array of objects!"))?;
        let Some(comments) = book.get(KEY_COMMENTS) else {
            continue;
        };
        let comments = comments
            .as_array()
            .ok_or_else(|| format!("comments need be an array: {book:?}"))?;
        let mut ratings = Vec::new();
        for comment in comments {
            let comment = comment
                .as_object()
                .ok_or_else(|| format!("expected comment '{comment:?}' to be an object"))?;
            let from = comment
                .get(KEY_FROM)
                .and_then(Value::as_str)
                .ok_or_else(|| format!("comment '{comment:?}' is missing {KEY_FROM} field!"))?;
            if !members_map.contains_key(from) {
                println!(
                    "warning: comment has an unknown author: {from}\n\tknown members: {:?}",
                    members_map.keys().collect::<Vec<_>>(),
                );
            }
            let Some(rating) = comment.get(KEY_RATING).and_then(value_to_f64) else {
                println!(
                    "warning: comment is missing a rating:\n{}",
                    to_string_pretty(comment).unwrap()
                );
                continue;
            };
            member_ratings
                .entry(from.to_string())
                .or_insert(Vec::new())
                .push(rating);
            ratings.push(rating);
        }
        let ratings_sum: f64 = ratings.iter().sum();
        let avg_rating = ratings_sum / ratings.len() as f64;
        book.insert(String::from(KEY_AVERAGE_RATING), json!(to_string_2_dec_places(avg_rating)));
        book.insert(String::from(KEY_RATING_PERCENT), json!(avg_rating * 20.0));
    }
    Ok(member_ratings)
}

fn value_to_f64(value: &Value) -> Option<f64> {
    match value {
        Value::Null | Value::Bool(_) | Value::Array(_) | Value::Object(_) => None,
        Value::Number(_) => value.as_f64(),
        Value::String(s) => if let Some(f) = s.parse::<f64>().ok() {
            Some(f)
        } else {
            None
        }
    }
}

fn read_gon_object(content_dir: &PathBuf, path: &str) -> Result<Value, String> {
    let src = std::fs::read_to_string(content_dir.join(path))
        .map_err(|_| format!("missing '{dir}/{path}'!", dir = content_dir.display()))?;

    let gon = gon::parse_str(&src).map_err(|e| format!("ill-formed {path}: {e}"))?.unwrap();
    Ok(gon.into())
}

fn read_file(content_dir: &PathBuf, path: &str) -> Result<String, String> {
    std::fs::read_to_string(content_dir.join(path))
        .map_err(|_| format!("missing '{dir}/{path}'!", dir = content_dir.display()))
}

fn render_file(
    dst_dir: &Path,
    handlebars: &Handlebars,
    root: &Value,
    template_name: &str,
    file_name: &str,
) {
    let first = handlebars
        .render(template_name, &root)
        .expect("handlebar error");
    std::fs::write(
        dst_dir.join(file_name),
        handlebars
            .render_template(&first, &root)
            .expect("handlebar error"),
    )
    .expect(file_name);
}

// Taken from https://stackoverflow.com/a/65192210
fn cp_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            cp_dir(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn to_string_2_dec_places(n: f64) -> String {
    format!("{:.2}", n)
}
