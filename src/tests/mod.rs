#[cfg(test)]
use super::rst_parser::{parse_images, parse_links};

#[test]
fn test_link_parser() {
    let mut input = String::from(
        "This is a paragraph that contains `a link`_.

    .. _a link: https://domain.invalida\n",
    );

    let output = parse_links(&mut input).unwrap();

    assert_eq!(output.trim_end(), "This is a paragraph that contains <a class=\"link\" href=\"https://domain.invalida\">a link</a>.")
}

#[test]
fn test_image_parser() {
    let input = ".. image:: cool/image/123.png
            :width: 60%
            :height: auto
            :alt: this is the alt text
        ";

    let output = parse_images(input).unwrap();

    assert_eq!(output.trim_end(), "<img src=\"cool/image/123.png\" alt=\"this is the alt text\" style=\"height;60%;width:60%;\">")
}
