#[cfg(test)]
use super::rst_parser::parse_links;

#[test]
fn test_link_parser() {
    let mut input = String::from(
        "This is a paragraph that contains `a link`_.

    .. _a link: https://domain.invalida\n",
    );

    let output = parse_links(&mut input);

    assert_eq!(output.trim_end(), "This is a paragraph that contains <a class=\"link\" href=\"https://domain.invalida\">a link</a>.")
}
