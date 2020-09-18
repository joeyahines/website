use regex::{Regex, Captures};
use std::collections::HashMap;

/// Renders RST links as HTML links
///
/// # Arguments
///
/// * `string` - input RST string
///
pub fn parse_links(string: &str) -> String {
    let re_link_ref = Regex::new(r"\n?.. _(.*): (.*)\n").unwrap();
    let mut link_map: HashMap<String, String> = HashMap::new();

    for cap in re_link_ref.captures_iter(string) {
        link_map.insert(String::from(&cap[1]), String::from(&cap[2]));
    }

    let text: String = re_link_ref.replace_all(string, "").to_string();

    let re_link = Regex::new(r"`(.*)`_").unwrap();

    let output = re_link.replace_all(&text, |cap: &Captures| {
        let link = match link_map.get(&cap[1]) {
            None => String::from(""),
            Some(link) => link.to_owned(),
        };

        format!("<a class=\"link\" href=\"{}\">{}</a>", link, &cap[1])
    });

    output.to_string()
}
