use crate::models::Details;
use select::document::Document;
use select::predicate::Name;

pub fn scraper(body: &str) -> Details {
    let title = get_title(&body);
    let links = get_links(&body);
    let scripts = get_content(&body, "script", "src");

    let details = Details {
        title,
        links,
        scripts,
    };

    details
}

fn get_links(html: &str) -> Vec<String> {
    let mut hyperlinks = Vec::new();
    let document = Document::from(html);
    let links = document.find(Name("a"));
    for link in links {
        match link.attr("href") {
            Some(href) => hyperlinks.push(href.to_owned()),
            None => (),
        };
    }

    hyperlinks
}

fn get_content(html: &str, tag: &str, attribute: &str) -> Vec<String> {
    let mut script_srcs = Vec::new();
    let document = Document::from(html);
    let scripts = document.find(Name(tag));
    for script in scripts {
        match script.attr(attribute) {
            Some(src) => script_srcs.push(src.to_owned()),
            None => (),
        };
    }

    script_srcs
}

fn get_title(html: &str) -> Option<String> {
    let document = Document::from(html);
    match document.find(Name("title")).next() {
        Some(title) => return Some(title.text()),
        None => return None,
    };
}
