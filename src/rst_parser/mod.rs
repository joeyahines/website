pub mod rst_error;

use regex::{Captures, Regex};
use rst_error::RSTError;
use std::collections::HashMap;
use std::convert::TryFrom;

/// RST Image
struct RSTImage {
    /// File location
    image_file: String,
    /// Height
    height: Option<String>,
    /// Width
    width: Option<String>,
    /// Alt Text
    alt: Option<String>,
    /// Align (not implemented)
    #[allow(dead_code)]
    align: Option<String>,
}

impl RSTImage {
    /// Convert to HTML img tag
    fn to_html(&self) -> String {
        let mut html = format!("<img src=\"{}\"", self.image_file);
        let mut style = "".to_string();

        if let Some(alt) = &self.alt {
            html = format!("{} alt=\"{}\"", html, alt);
        }

        if let Some(height) = &self.height {
            style = format!("{}height;{};", style, height);
        }
        if let Some(width) = &self.width {
            style = format!("{}width:{};", style, width);
        }

        html = format!("{} style=\"{}\">", html, style);

        html
    }
}

impl TryFrom<&Captures<'_>> for RSTImage {
    type Error = RSTError;
    /// Convert from a regex capture to a RSTImage
    ///
    /// # Arguments
    ///
    /// * cap - regex capture
    fn try_from(cap: &Captures<'_>) -> Result<Self, Self::Error> {
        let image = Self {
            image_file: cap
                .name("file_name")
                .map(|m| m.as_str().to_string())
                .ok_or(RSTError::NotFound("file_name"))?,
            height: cap.name("width").map(|m| m.as_str().to_string()),
            width: cap.name("width").map(|m| m.as_str().to_string()),
            alt: cap.name("alt").map(|m| m.as_str().to_string()),
            align: cap.name("align").map(|m| m.as_str().to_string()),
        };

        Ok(image)
    }
}

/// Renders RST links as HTML links
///
/// # Arguments
///
/// * `string` - input RST string
pub fn parse_links(string: &str) -> Result<String, RSTError> {
    let re_link_ref = Regex::new(r"\n?.. _(.*): (.*)\n")?;
    let mut link_map: HashMap<String, String> = HashMap::new();

    for cap in re_link_ref.captures_iter(string) {
        link_map.insert(String::from(&cap[1]), String::from(&cap[2]));
    }

    let text: String = re_link_ref.replace_all(string, "").to_string();

    let re_link = Regex::new(r"`(.*)`_")?;

    let output = re_link.replace_all(&text, |cap: &Captures| {
        let link = match link_map.get(&cap[1]) {
            None => String::from(""),
            Some(link) => link.to_owned(),
        };

        format!("<a class=\"link\" href=\"{}\">{}</a>", link, &cap[1])
    });

    Ok(output.to_string())
}

/// Renders RST images as HTML embedded images
///
/// # Arguments
///
/// * `string` - input rst string
pub fn parse_images(string: &str) -> Result<String, RSTError> {
    let re_image = Regex::new(
        r".. image:: (?P<file_name>.*)\n( *((:height: (?P<height>.*))|(:width: (?P<width>.*))|(:scale: (?P<scale>.*%))|(:alt: (?P<alt>.*))|(:align: (?P<align>.*)))\n)*",
    )?;

    Ok(re_image
        .replace_all(string, |cap: &Captures| {
            RSTImage::try_from(cap).unwrap().to_html()
        })
        .to_string())
}
