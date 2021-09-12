#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod tests;
mod rst_parser;

use crate::rst_parser::parse_images;
use regex::Regex;
use rocket::Request;
use rocket_dyn_templates::Template;
use rst_parser::parse_links;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::{fs, io};

type PageResult<T> = std::result::Result<T, JSiteError>;

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

#[derive(Clone, Debug)]
enum JSiteError {
    PageNotFound(PageNotFoundError),
    IOError,
}

impl std::convert::From<PageNotFoundError> for JSiteError {
    fn from(e: PageNotFoundError) -> Self {
        JSiteError::PageNotFound(e)
    }
}

impl std::convert::From<std::io::Error> for JSiteError {
    fn from(_: Error) -> Self {
        JSiteError::IOError
    }
}

#[derive(Serialize)]
struct SiteFile {
    rank: u32,
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
    get_pages("static/raw_rst", &mut links).ok();

    map.insert("links", links);
    Template::render("index", &map)
}

/// Gets all the raw rst pages contained in a directory
///
/// The order of the vector is determined by OS. Ordering can be set by prepending the file name
/// with a number. Files that start with lower numbers are placed earlier in the list.
///
/// # Arguments
/// * `path` - the path to look for pages in
/// * `pages` - A vector where found pages will be inserted
fn get_pages(path: &str, pages: &mut Vec<SiteFile>) -> io::Result<()> {
    let re = Regex::new(r"(?P<rank>^\d*)(?P<link_name>.+)").unwrap();

    // Find all files in the directory
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = match path.file_stem() {
            Some(name) => name,
            None => continue,
        };

        let file_name = match file_name.to_str() {
            Some(name) => name,
            None => continue,
        };

        if let Some(caps) = re.captures(file_name) {
            let link_name = &caps["link_name"];
            let rank = &caps["rank"];

            let rank: u32 = if rank.is_empty() {
                std::u32::MAX
            } else {
                match rank.parse() {
                    Ok(r) => r,
                    Err(_) => std::u32::MAX,
                }
            };

            let site_file = SiteFile {
                rank,
                file_name: file_name.to_string(),
                link_name: link_name.to_string(),
                path: entry.path(),
            };

            pages.push(site_file);
        }
    }

    pages.sort_by(|a, b| a.rank.cmp(&b.rank));

    Ok(())
}

/// Gets a page matching `page_name` in directory `path`
///
/// # Arguments
/// * `path` - path to search in
/// * `page_name` - file to look for
fn get_page(path: &Path) -> PageResult<SiteFile> {
    let file_name = path.file_name().ok_or(PageNotFoundError)?;
    let file_name = file_name.to_str().ok_or(PageNotFoundError)?.to_string();
    if path.exists() {
        return Ok(SiteFile {
            rank: 0,
            file_name: file_name.clone(),
            link_name: file_name,
            path: path.to_path_buf(),
        });
    } else {
        let mut dir_path = path.to_path_buf();
        dir_path.pop();

        for entry in dir_path.read_dir()? {
            let entry = entry?;
            let entry_name = entry.file_name().into_string().unwrap();

            if entry_name.contains(&file_name) {
                return Ok(SiteFile {
                    rank: 0,
                    file_name: entry_name,
                    link_name: file_name,
                    path: entry.path(),
                });
            }
        }
    }

    Err(JSiteError::from(PageNotFoundError))
}

fn error_page(page: &str) -> Template {
    let mut map = HashMap::new();
    map.insert("error_page", page);
    Template::render("404", map)
}

/// Returns a rendered template of a raw rst page if it exists
///
/// # Arguments
/// * `page` - path to page
#[get("/about/<page..>")]
fn rst_page(page: PathBuf) -> Template {
    let mut path = PathBuf::from("static/raw_rst");
    path.push(page);

    // Try and get the page
    let site_page = match get_page(path.as_path()) {
        Ok(site_page) => site_page,
        Err(_) => {
            return error_page(path.to_str().unwrap());
        }
    };

    if site_page.path.is_dir() {
        // If the file is a directory, list its contents instead
        let mut map = HashMap::new();
        let mut sub_files: Vec<SiteFile> = Vec::new();
        match get_pages(site_page.path.to_str().unwrap(), &mut sub_files) {
            Ok(_) => (),
            Err(_) => return error_page(&site_page.link_name),
        }

        let page_data = PageData {
            links: sub_files,
            site_file: site_page,
        };

        map.insert("page_data", page_data);
        Template::render("listing", &map)
    } else {
        // Else, render the RST page
        let mut map = HashMap::new();
        let contents = match fs::read_to_string(site_page.path) {
            Ok(contents) => contents,
            Err(_) => {
                let mut map = HashMap::new();
                map.insert("error_page", site_page.link_name);
                return Template::render("404", map);
            }
        };

        // Render links
        let mut contents = parse_links(&contents).unwrap();
        contents = parse_images(contents.as_str()).unwrap();

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
/// * `req` - information on the original request
#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();

    map.insert("error_page", String::from(req.uri().path().as_str()));

    Template::render("404", &map)
}

/// Launches website
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, rst_page])
        .mount("/static", rocket::fs::FileServer::from("static"))
        .attach(Template::fairing())
        .register("/",catchers![not_found])
}
