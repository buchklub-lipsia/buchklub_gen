mod model;

use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    str::FromStr,
};

use chrono::Local;
use handlebars::{Handlebars, TemplateError};
use serde_json::{json, Map, Number, Value};

pub const DATE_FORMAT: &str = "%d.%m.%Y %H:%M";

fn main() -> Result<(), TemplateError> {
    let content_dir = PathBuf::from_str("content").expect("failed to init root content path");
    let member_file =
        File::open(content_dir.join("members.json")).expect("members.json doesn't exist");
    let global_file =
        File::open(content_dir.join("global.json")).expect("gloal.json doesn't exist");
    let books_file = File::open(content_dir.join("books.json")).expect("books.json doesn't exist");
    let mut members: Value = serde_json::from_reader(member_file).expect("ill-formed members.json");
    let mut global: Value = serde_json::from_reader(global_file).expect("ill-formed global.json");
    let mut books: Value = serde_json::from_reader(books_file).expect("ill-formed books.json");
    let mut root = Map::new();
    let global_map = global.as_object_mut().expect("global has to be an object");
    global_map.insert(
        String::from("build_time"),
        json!(format!("{}", Local::now().format(DATE_FORMAT))),
    );
    global_map.insert(
        String::from("header"),
        json!(std::fs::read_to_string(content_dir.join("header.html")).expect("header.html")),
    );
    global_map.insert(
        String::from("footer"),
        json!(std::fs::read_to_string(content_dir.join("footer.html")).expect("footer.html")),
    );
    // Calculate average ratings
    let members_obj = members.as_object_mut().expect("members to be an object");
    let ratings: Vec<_> = books
        .as_array()
        .expect("books to be an array")
        .iter()
        .flat_map(|book| {
            book.as_object()
                .expect("book to be an object")
                .get("comments")
                .expect("book to have comments")
                .as_array()
                .expect("comments to be an array")
        })
        .map(|comment| {
            let comment = comment.as_object().expect("comments to contain objects");
            (comment.get("from").and_then(Value::as_str).expect("comment to have string from field"), comment.get("rating").and_then(Value::as_str).expect("comment to have rating field"))
        }).collect();

    for (id, member) in members_obj {
        let their_ratings = ratings.iter().filter(|(i, _)| i == id).filter_map(|(_, rating)| rating.parse::<f64>().ok()).collect::<Vec<_>>();
        member.as_object_mut().expect("member to be an object").insert(String::from("average-rating"), json!(to_string_2_dec_places(their_ratings.iter().sum::<f64>() / their_ratings.len() as f64)));
    }

    // average rating per book; unwraps because of prior checks
    for book in books.as_array_mut().unwrap() {
        let book = book.as_object_mut().unwrap();
        let ratings: Vec<_> = book.get("comments").unwrap().as_array().unwrap().iter()
            .map(Value::as_object)
            .map(Option::unwrap)
            .filter_map(|c| c.get("rating").and_then(Value::as_str).and_then(|s| s.parse::<f64>().ok()))
            .collect();
        book.insert(String::from("average-rating"), if ratings.is_empty() { json!("n. a.") } else { json!(to_string_2_dec_places(ratings.iter().sum::<f64>() / ratings.len() as f64)) });
    }

    let members_list = Value::Array(
        members
            .clone()
            .as_object_mut()
            .expect("members has to be an object")
            .iter_mut()
            .map(|(id, value)| {
                value
                    .as_object_mut()
                    .unwrap()
                    .insert(String::from("id"), json!(id));
                value.clone()
            })
            .collect(),
    );
    root.insert(String::from("members"), members);
    root.insert(String::from("members-list"), members_list);
    root.insert(String::from("global"), global);
    root.insert(String::from("books"), books);

    let root = Value::Object(root);
    let mut handlebars = Handlebars::new();
    handlebars.register_partial(
        "book",
        std::fs::read_to_string(content_dir.join("book.html")).expect("book.html"),
    )?;
    handlebars
        .register_template_file("index", "content/index.html")
        .map_err(|e| {
            println!("{e}");
            e
        })?;
    handlebars.register_template_file("books", "content/books.html")?;
    handlebars.register_template_file("members", "content/members.html")?;
    handlebars.register_template_file("statute", "content/statute.html")?;
    handlebars.register_template_file("contact", "content/contact.html")?;
    handlebars.register_template_file("misc", "content/misc.html")?;

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
    Ok(())
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
