mod config;
mod error;

use axum::error_handling::HandleErrorLayer;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{BoxError, Router};
use axum_extra::routing::SpaRouter;
use error::{JSiteError, PageResult};
use pulldown_cmark::html::push_html;
use pulldown_cmark::{Options, Parser};
use regex::Regex;
use serde::Serialize;
use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use structopt::StructOpt;
use tera::{Context, Tera};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::config::SiteArgs;

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

/// Returns the rendered template of the index page of the website. This includes links and md
/// pages included in `static/raw_md`
async fn index(State(state): State<Arc<Tera>>) -> PageResult<impl IntoResponse> {
    let mut ctx = Context::new();
    let mut links: Vec<SiteFile> = Vec::new();

    // Get the links to display on the main page
    get_pages("static/raw_md", &mut links).await?;

    ctx.insert("links", &links);
    Ok(Html(state.render("index.html.tera", &ctx)?))
}

/// Gets all the raw md pages contained in a directory
///
/// The order of the vector is determined by OS. Ordering can be set by prepending the file name
/// with a number. Files that start with lower numbers are placed earlier in the list.
///
/// # Arguments
/// * `path` - the path to look for pages in
/// * `pages` - A vector where found pages will be inserted
async fn get_pages(path: &str, pages: &mut Vec<SiteFile>) -> PageResult<()> {
    let re = Regex::new(r"(?P<rank>^\d*)(?P<link_name>.+)").unwrap();

    // Find all files in the directory
    let mut dir = tokio::fs::read_dir(path).await?;
    while let Some(entry) = dir.next_entry().await? {
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

            let rank = match rank.parse() {
                Ok(r) => r,
                Err(_) => u32::MAX,
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
fn get_page(path: &std::path::Path) -> PageResult<SiteFile> {
    let file_name = path
        .file_name()
        .ok_or(JSiteError::PageNotFound(path.to_path_buf()))?;
    let file_name = file_name
        .to_str()
        .ok_or(JSiteError::PageNotFound(path.to_path_buf()))?
        .to_string();

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

    Err(JSiteError::PageNotFound(path.to_path_buf()))
}

/// Returns a rendered template of a raw md page if it exists
///
/// # Arguments
/// * `page` - path to page
async fn md_page(tera: State<Arc<Tera>>, Path(page): Path<PathBuf>) -> PageResult<Html<String>> {
    let mut path = PathBuf::from("static/raw_md");
    path.push(page);

    // Try and get the page
    let site_page = match get_page(path.as_path()) {
        Ok(site_page) => site_page,
        Err(_) => {
            return error_page(&tera, path.to_str().unwrap()).await;
        }
    };

    let page = if site_page.path.is_dir() {
        // If the file is a directory, list its contents instead
        let mut map = Context::new();
        let mut sub_files: Vec<SiteFile> = Vec::new();
        match get_pages(site_page.path.to_str().unwrap(), &mut sub_files).await {
            Ok(_) => (),
            Err(_) => return error_page(&tera, &site_page.link_name).await,
        }

        let page_data = PageData {
            links: sub_files,
            site_file: site_page,
        };

        map.insert("page_data", &page_data);
        tera.render("listing.html.tera", &map)?
    } else {
        // Else, render the MD page
        let mut map = Context::new();
        let contents = match tokio::fs::read_to_string(site_page.path.clone()).await {
            Ok(contents) => contents,
            Err(_) => return error_page(&tera, site_page.path.to_str().unwrap()).await,
        };

        let options = Options::all();
        let parser = Parser::new_ext(&contents, options);

        let mut html_output = String::new();
        push_html(&mut html_output, parser);

        map.insert("page", &site_page.link_name);
        map.insert("content", &html_output);
        tera.render("md_page.html.tera", &map)?
    };

    Ok(Html(page))
}

/// Build error page
async fn error_page(tera: &Tera, page: &str) -> PageResult<Html<String>> {
    let mut map = Context::new();
    map.insert("error_page", page);
    Ok(Html(tera.render("404.html.tera", &map)?))
}

/// Handle server errors
async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
}

/// Launches website
#[tokio::main]
async fn main() {
    let args = SiteArgs::from_args();

    // Use globbing
    let tera = match Tera::new("templates/*.tera") {
        Ok(t) => Arc::new(t),
        Err(e) => {
            println!("Parsing error(s): {}", e);
            return;
        }
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/about/*path", get(md_page))
        .merge(SpaRouter::new("/static", "static"))
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http()),
        )
        .with_state(tera);

    println!("listening on {}", args.serve_at);
    axum::Server::bind(&args.serve_at)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
