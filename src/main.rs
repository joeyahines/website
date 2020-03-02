#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

mod tests;
mod rst_parser;

use rst_parser::parse_links;
use std::collections::HashMap;
use rocket::Request;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use std::{fs, io};
use std::path::PathBuf;
use std::error;
use std::fmt;

type PageResult<T> = std::result::Result<T, PageNotFoundError>;

#[derive(Clone, Debug)]
struct PageNotFoundError;

impl fmt::Display for PageNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Page not found")
    }
}

impl error::Error for PageNotFoundError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Serialize)]
struct SiteFile {
    file_name: String,
    link_name: String,
    path: PathBuf,
}

#[derive(Serialize)]
struct PageData {
    site_file: SiteFile,
    links: Vec<SiteFile>,
}

/// Returns the rendered template of the index page of the website. This includes links and rst
/// pages included in `static/raw_rst`
#[get("/")]
fn index() -> Template {
    let mut map: HashMap<&str, Vec<SiteFile>> = HashMap::new();
    let mut links: Vec<SiteFile> = Vec::new();

    // Get the links to display on the main page
    match get_pages("static/raw_rst", &mut links) {
       Err(_) => (),
        Ok(_) => (),
    }

    map.insert("links", links);
    Template::render("index", &map)
}

/// Gets all the raw rst pages contained in static/raw_rst/
///
/// The rst page can start with a number
///
/// # Arguments
///
/// * `links` - A reference to a vector of string to insert the links into
fn get_pages(path: &str, links: &mut Vec<SiteFile>) -> io::Result<()> {
    // Gather all of the rst files in static/raw_rst/
    let mut entries: Vec<PathBuf> =  fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // Sort so they are always in the same order
    entries.sort();

    // Find all files in the directory
    for entry in entries {
        let file_name = entry.file_stem().unwrap().to_str().unwrap();
        let link_name;
        if file_name.chars().next().unwrap().is_numeric() {
            link_name = &file_name[1..];
        }
        else {
            link_name = file_name;
        }

        let rst_file = SiteFile {
            file_name: String::from(file_name),
            link_name: String::from(link_name),
            path: entry.to_owned()
        };

        links.push(rst_file);
    }

    Ok(())
}

/// Gets a page matching `page_name` in directory `path`
///
/// # Arguments
///
/// * `path` - path to search in
/// * `page_name` - file to look for
fn get_page(path: &str, page_name: &str) -> Result<SiteFile, PageNotFoundError> {
    let mut pages: Vec<SiteFile> = Vec::new();

    // Get pages
    match get_pages(path, &mut pages) {
        Err(_) => return Err(PageNotFoundError),
        Ok(_) => (),
    };

    // Look for the page in the directory
    for page in pages {
        if page.link_name.eq_ignore_ascii_case(page_name) {
            return Ok(page)
        }
    }

    Err(PageNotFoundError)
}

/// Returns a rendered template of a raw rst page if it exists
///
/// # Arguments
///
/// * `page` - a string containing the name of the rst file to look for
#[get("/about/<page..>")]
fn rst_page(page: PathBuf) -> Template {
    // Try and get the page
    let site_page = match get_page(format!("static/raw_rst/{}", page.parent().unwrap().to_str().unwrap()).as_str(),  &page.file_name().unwrap().to_str().unwrap()) {
        Ok(site_page) => site_page,
        Err(_) => {
            let mut map = HashMap::new();
            map.insert("error_page", page);
            return Template::render("404", map)
        }
    };

    if site_page.path.is_dir() {
        // If the file is a directory, list its contents instead
        let mut map = HashMap::new();
        let mut sub_files: Vec<SiteFile> = Vec::new();
        match get_pages(site_page.path.to_str().unwrap(), &mut sub_files) {
            Err(_) => (),
            Ok(_) => (),
        };

        let page_data = PageData {
            links: sub_files,
            site_file: site_page
        };

        map.insert("page_data", page_data);
        return Template::render("listing", &map);
    }
    else {
        // Else, render the RST page
        let mut map = HashMap::new();
        let contents = match fs::read_to_string(site_page.path) {
            Ok(contents) => contents,
            Err(_) => {
                let mut map = HashMap::new();
                map.insert("error_page", page);
                return Template::render("404", map)
            },
        };

        // Render links
        let mut contents = parse_links(&contents);

        // Ensure render will look good
        contents = contents.replace("\n", "<br>");
        contents = contents.replace("  ", "&nbsp;&nbsp;");

        map.insert("page", site_page.link_name);
        map.insert("content", contents);
        Template::render("rst_page", &map)
    }

}


/// Catches 404 errors and displays an error message
///
/// #Arguments
///
/// * `req` - information on the original request
#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();

    map.insert("error_page", String::from(req.uri().path()));

    Template::render("404", &map)
}


/// Launches website
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, rst_page], )
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .register(catchers![not_found])
}

/// Main
fn main() {
    rocket().launch();
}