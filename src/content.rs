use mdsycx::ParseRes;
use serde::Deserialize;

use std::{
    collections::HashMap,
    fmt::{Display, Write},
    fs,
    path::PathBuf,
    str::FromStr,
    sync::LazyLock,
};

use crate::{
    docs::{DocPage, DocsSource, DOCS_SOURCES},
    Routes, DOCS_DIR,
};

/// Frontmatter for a blog post.
#[derive(Debug, Clone, Deserialize)]
pub struct PostFrontmatter {
    pub title: String,
    pub description: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub date: Date,
}

/// Frontmatter for a documentation page.
#[derive(Debug, Clone, Deserialize)]
pub struct DocFrontmatter {
    /// The title of the doc page.
    pub title: String,
    /// Any subsections of the doc page.
    #[serde(default)]
    pub subsections: Vec<String>,
}

/// Represents a date in the format "YYYY-MM-DD".
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: u32,
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.year, self.month, self.day).cmp(&(other.year, other.month, other.day)))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.year, self.month, self.day).cmp(&(other.year, other.month, other.day))
    }
}

impl FromStr for Date {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-').map(|x| x.parse().unwrap());
        Ok(Date {
            year: parts.next().ok_or("could not parse year")?,
            month: parts.next().ok_or("could not parse month")?,
            day: parts.next().ok_or("could not parse date")?,
        })
    }
}

/// Deserialize date in format "YYYY-MM-DD"
fn deserialize_date<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // TODO: error handling
    Ok(Date::from_str(&s).unwrap())
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        static MONTHS: &[&str] = &[
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];

        let month = MONTHS.get(self.month as usize - 1).unwrap_or(&"Error");

        write!(f, "{month} {day}, {year}", day = self.day, year = self.year)
    }
}

/// Represents the top-level sections of the book.
///
/// This is parsed from `sections.json`.
#[derive(Debug, Clone, Deserialize)]
pub struct SectionsJson {
    pub sections: Vec<String>,
}

fn parse_doc<T>(path: &std::path::Path) -> ParseRes<T>
where
    T: for<'a> Deserialize<'a> + 'static,
{
    let full_path = path.with_extension("md");
    let md = fs::read_to_string(&full_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", full_path.display()));
    mdsycx::parse(&md)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err:?}", full_path.display()))
}

pub struct Book {
    pub source: DocsSource,
    pub docs: HashMap<DocPage, ParseRes<DocFrontmatter>>,
    pub index: BookIndex,
}

impl Book {
    pub fn switch_url(&self, page: &DocPage) -> String {
        let target = if self.docs.contains_key(page) {
            page
        } else {
            &self
                .index
                .sections
                .first()
                .expect("book must have a section")
                .path
        };
        self.source.url(target)
    }
}

pub static BOOKS: LazyLock<Vec<Book>> = LazyLock::new(|| {
    DOCS_SOURCES
        .iter()
        .map(|source| {
            let root = if source.is_latest() {
                PathBuf::from(DOCS_DIR).join("next")
            } else {
                PathBuf::from(source.checkout).join("docs/next")
            };
            let sections: SectionsJson = serde_json::from_str(
                &fs::read_to_string(root.join("sections.json"))
                    .unwrap_or_else(|err| panic!("failed to load {}: {err}", root.display())),
            )
            .expect("invalid sections.json");
            let mut docs = HashMap::new();
            let mut index = BookIndex {
                sections: Vec::new(),
            };
            for section in sections.sections {
                assert!(
                    !DOCS_SOURCES
                        .iter()
                        .any(|s| !s.is_latest() && s.slug == section),
                    "section name conflicts with a version prefix: {section}"
                );
                let page = DocPage::new(section.clone(), None);
                let mut parsed = parse_doc::<DocFrontmatter>(&root.join(&section));
                let mut subsections = Vec::new();
                for subsection in &parsed.front_matter.subsections {
                    let path = DocPage::new(section.clone(), Some(subsection.clone()));
                    let mut child =
                        parse_doc::<DocFrontmatter>(&root.join(&section).join(subsection));
                    assert!(
                        child.front_matter.subsections.is_empty(),
                        "subsections cannot have subsections"
                    );
                    rewrite_doc_links(&mut child, *source);
                    subsections.push(BookItem {
                        title: child.front_matter.title.clone(),
                        path: path.clone(),
                    });
                    docs.insert(path, child);
                }
                rewrite_doc_links(&mut parsed, *source);
                index.sections.push(BookSection {
                    title: parsed.front_matter.title.clone(),
                    subsections,
                    path: page.clone(),
                });
                docs.insert(page, parsed);
            }
            assert!(!index.sections.is_empty(), "empty book: {}", source.slug);
            Book {
                source: *source,
                docs,
                index,
            }
        })
        .collect()
});

pub fn book(source: DocsSource) -> &'static Book {
    BOOKS
        .iter()
        .find(|book| book.source == source)
        .expect("unknown docs source")
}

/// Rewrite parsed href attributes, never code examples or arbitrary Markdown text.
/// mdsycx exposes its body through serde, but keeps the events field private.
fn rewrite_doc_links(parsed: &mut ParseRes<DocFrontmatter>, source: DocsSource) {
    if source.is_latest() {
        return;
    }
    let mut body = serde_json::to_value(&parsed.body).unwrap();
    for event in body["events"].as_array_mut().expect("mdsycx body events") {
        if let Some(attr) = event.get_mut("Attr").and_then(|v| v.as_array_mut()) {
            if attr[0] == "href" {
                if let Some(href) = attr[1].as_str() {
                    attr[1] = rewrite_doc_href(href, source).into();
                }
            }
        }
    }
    parsed.body = serde_json::from_value(body).unwrap();
}

fn rewrite_doc_href(href: &str, source: DocsSource) -> String {
    // Same-site absolute URLs should behave like root-relative documentation links.
    let local = href.strip_prefix("https://sycamore.dev").unwrap_or(href);
    let Some(rest) = local.strip_prefix("/book/") else {
        return href.to_owned();
    };
    let first = rest.split(['/', '?', '#']).next().unwrap_or("");
    // An explicitly versioned link expresses an intentional cross-version reference.
    if DOCS_SOURCES
        .iter()
        .any(|s| !s.is_latest() && s.slug == first)
    {
        return href.to_owned();
    }
    format!("/book/{}/{rest}", source.slug)
}

/// Stores all the information of the structure of the book.
#[derive(Debug, Clone)]
pub struct BookIndex {
    pub sections: Vec<BookSection>,
}

#[derive(Debug, Clone)]
pub struct BookSection {
    pub title: String,
    pub subsections: Vec<BookItem>,
    pub path: DocPage,
}

#[derive(Debug, Clone)]
pub struct BookItem {
    pub title: String,
    pub path: DocPage,
}

pub static POSTS: std::sync::LazyLock<HashMap<String, ParseRes<PostFrontmatter>>> =
    LazyLock::new(|| {
        let mut posts = HashMap::new();

        let post_dir = PathBuf::from(DOCS_DIR).join("posts");
        for entry in fs::read_dir(post_dir).expect("failed to read posts directory") {
            let entry = entry.expect("failed to read post entry");
            let path = entry.path();
            let name = path
                .file_stem()
                .expect("failed to get file stem")
                .to_string_lossy();

            let post = parse_doc::<PostFrontmatter>(
                &PathBuf::from(DOCS_DIR).join("posts").join(name.as_ref()),
            );

            posts.insert(name.to_string(), post);
        }

        posts
    });

pub fn get_static_paths() -> Vec<(Routes, String)> {
    let mut paths = vec![];

    paths.push((Routes::Index, "/index.html".to_string()));
    paths.push((Routes::NotFound, "/404.html".to_string()));

    for post in POSTS.keys() {
        paths.push((Routes::Post(post.clone()), format!("/post/{post}.html")));
    }

    for book in BOOKS.iter() {
        for page in book.docs.keys() {
            let url = book.source.url(page);
            let segments = url
                .trim_start_matches("/book/")
                .split('/')
                .map(str::to_owned)
                .collect();
            paths.push((Routes::Book(segments), format!("{url}.html")));
        }
    }
    paths.sort_by(|a, b| a.1.cmp(&b.1));

    paths
}

/// Generate an XML sitemap file.
pub fn generate_sitemap_xml() -> Result<String, std::fmt::Error> {
    static BASE_URL: &str = "https://sycamore.dev";

    let paths = get_static_paths();
    let mut buf = String::new();

    write!(
        &mut buf,
        r#"<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#
    )?;

    for (route, path) in paths {
        if !route.is_indexable() {
            continue;
        }
        let path = path.strip_suffix(".html").expect("should be an html page");
        let path = if path == "/index" { "/" } else { path };
        let loc = format!("{BASE_URL}{path}");

        write!(&mut buf, r#"<url><loc>{loc}</loc></url>"#)?;
    }

    write!(&mut buf, r#"</urlset>"#)?;

    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        docs::resolve_book,
        shell::{App, Shell},
    };
    use sycamore::prelude::*;
    use sycamore_router::Route;

    #[test]
    fn every_book_page_has_a_unique_static_path_and_component() {
        let paths = get_static_paths();
        let mut files = std::collections::HashSet::new();
        let mut ids = std::collections::HashSet::new();
        for (route, file) in &paths {
            assert!(files.insert(file));
            let url = file.strip_suffix(".html").unwrap();
            if matches!(route, Routes::Book(_)) {
                assert_eq!(Routes::default().match_path(url), *route);
            }
        }
        for book in BOOKS.iter() {
            for page in book.docs.keys() {
                let url = book.source.url(page);
                assert!(files.contains(&format!("{url}.html")));
                assert!(ids.insert(book.source.component_id(page)));
                let Routes::Book(segments) = Routes::default().match_path(&url) else {
                    panic!("not a book route")
                };
                assert_eq!(resolve_book(&segments), Some((book.source, page.clone())));
            }
        }
    }

    #[test]
    fn version_switch_preserves_existing_pages_and_falls_back() {
        for book in BOOKS.iter() {
            let first = &book.index.sections[0].path;
            assert_eq!(book.switch_url(first), book.source.url(first));
            assert_eq!(
                book.switch_url(&DocPage::new("missing-page".into(), None)),
                book.source.url(first)
            );
        }
    }

    #[test]
    fn sitemap_contains_latest_but_no_archived_books() {
        let sitemap = generate_sitemap_xml().unwrap();
        assert!(sitemap.contains("<loc>https://sycamore.dev/</loc>"));
        assert!(!sitemap.contains("/404"));
        for book in BOOKS.iter() {
            for page in book.docs.keys() {
                let entry = format!("<loc>https://sycamore.dev{}</loc>", book.source.url(page));
                assert_eq!(sitemap.contains(&entry), book.source.is_latest());
            }
        }
    }

    #[test]
    fn markdown_links_stay_versioned_without_changing_code_or_external_links() {
        for source in DOCS_SOURCES.iter().copied().filter(|s| !s.is_latest()) {
            let href = "/book/introduction/installation?example=1#heading";
            assert_eq!(
                rewrite_doc_href(href, source),
                format!(
                    "/book/{}/introduction/installation?example=1#heading",
                    source.slug
                )
            );
            for unchanged in [
                "#heading",
                "https://example.com/book/introduction",
                "/post/news",
                "/docs/v0.8/getting_started/installation",
            ] {
                assert_eq!(rewrite_doc_href(unchanged, source), unchanged);
            }
            let versioned = source.url(&DocPage::new("introduction".into(), None));
            assert_eq!(rewrite_doc_href(&versioned, source), versioned);
            let md = "---\ntitle: Test\n---\n[Book](/book/introduction)\n\n`/book/introduction`";
            let mut parsed: ParseRes<DocFrontmatter> = mdsycx::parse(md).unwrap();
            rewrite_doc_links(&mut parsed, source);
            let body = serde_json::to_value(&parsed.body).unwrap();
            let events = body["events"].as_array().unwrap();
            assert!(events.iter().any(|e| e.get("Attr")
                == Some(&serde_json::json!([
                    "href",
                    format!("/book/{}/introduction", source.slug)
                ]))));
            assert!(events
                .iter()
                .any(|e| e.get("Text") == Some(&serde_json::json!("/book/introduction"))));
        }
    }

    #[tokio::test]
    async fn rendered_books_have_correct_metadata_navigation_and_source_links() {
        for book in BOOKS.iter() {
            for page in book.docs.keys() {
                let url = book.source.url(page);
                let route = Routes::default().match_path(&url);
                let html = sycamore::render_to_string_await_suspense(|| {
                    view! {
                        Shell(initial_route=route.clone()) {
                            sycamore_router::StaticRouter(route=route, view=App)
                        }
                    }
                })
                .await;
                let head = html.split("</head>").next().unwrap();
                assert_eq!(
                    head.contains("content=\"noindex\""),
                    !book.source.is_latest(),
                    "{url}"
                );
                assert_eq!(
                    head.contains("rel=\"canonical\""),
                    book.source.is_latest(),
                    "{url}"
                );
                if book.source.is_latest() {
                    assert!(
                        head.contains(&format!("https://sycamore.dev{url}")),
                        "{url}"
                    );
                }
                assert!(html.contains(&book.source.edit_url(page)), "{url}");
                assert!(html.contains("sycamore-rs.netlify.app/docs/v0.8/"), "{url}");
                assert!(html.contains("sycamore-rs.netlify.app/docs/v0.7/"), "{url}");
                for section in &book.index.sections {
                    assert!(
                        html.contains(&format!("href=\"{}\"", book.source.url(&section.path))),
                        "{url}"
                    );
                }
                assert!(html.contains(&book.source.component_id(page)), "{url}");
            }
        }
    }
}
