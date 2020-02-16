#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;

use rocket::Request;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use std::{fs, io};
use std::path::PathBuf;

#[derive(Serialize)]
struct MDFile {
    file_name: String,
    link_name: String,
}

/// Returns the rendered template of the index page of the website. This includes links and md
/// pages included in `static/raw_md`
#[get("/")]
fn index() -> Template {

    let mut map: HashMap<&str, Vec<MDFile>> = HashMap::new();
    let mut links: Vec<MDFile> = Vec::new();

    match get_pages(&mut links) {
       Err(_) => (),
        Ok(_) => (),
    }

    map.insert("links", links);
    Template::render("index", &map)
}

/// Gets all the raw md pages contained in static/raw_md/
///
/// The md page can start with a number
///
/// # Arguments
///
/// * `links` - A reference to a vector of string to insert the links into
fn get_pages(links: &mut Vec<MDFile>) -> io::Result<()> {
    // Gather all of the md files in static/raw_md/
    let mut entries: Vec<PathBuf> =  fs::read_dir("static/raw_md/")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // Sort so they are always in the same order
    entries.sort();

    //
    for entry in entries {
        let file_name = entry.file_stem().unwrap().to_str().unwrap();
        let link_name;
        if file_name.chars().next().unwrap().is_numeric() {
            link_name = &file_name[1..];
        }
        else {
            link_name = file_name;
        }

        let md_file = MDFile {
            file_name: String::from(file_name),
            link_name: String::from(link_name),
        };

        links.push(md_file);
    }

    Ok(())
}

/// Returns a rendered template of a raw md page if it exists
///
/// #Arguments
///
/// * `page` - a string containing the name of the md file to look for
#[get("/<page>")]
fn md_page(page: String) -> Template {
    let mut map = HashMap::new();
    let mut md_files: Vec<MDFile> = Vec::new();

    match get_pages(&mut md_files) {
      Err(_) => (),
        Ok(_) => (),
    };

    let mut file_name = String::new();
    for md_file in md_files {
        if md_file.link_name.eq_ignore_ascii_case(page.as_str()) {
            file_name = md_file.file_name.clone();
        }
    }

    let mut contents = match fs::read_to_string(format!("static/raw_md/{}.md", file_name)) {
        Ok(contents) => contents,
        Err(_) => {
            map.insert("error_page", page);
            return Template::render("404", map)
        },
    };

    contents = contents.replace("\n", "<br>");
    contents = contents.replace("  ", "&nbsp;&nbsp;");

    map.insert("page", page);
    map.insert("md_data", contents);
    Template::render("md_page", &map)
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();

    map.insert("error_page", String::from(req.uri().path()));

    Template::render("404", &map)
}


fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, md_page], )
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}