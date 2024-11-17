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

use crate::{Routes, DOCS_DIR};

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

pub static SECTIONS_JSON: LazyLock<SectionsJson> = LazyLock::new(|| {
    let sidebar_json = fs::read_to_string(format!("{DOCS_DIR}/next/sections.json"))
        .expect("failed to read sections.json");
    serde_json::from_str(&sidebar_json).expect("failed to parse sections.json")
});

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocPage(String, Option<String>);

impl DocPage {
    pub fn new(section: String, doc: Option<String>) -> Self {
        Self(section, doc)
    }

    pub fn section(&self) -> &str {
        &self.0
    }

    pub fn subsection(&self) -> Option<&str> {
        self.1.as_deref()
    }
}

fn parse_doc<T>(path: &str) -> ParseRes<T>
where
    T: for<'a> Deserialize<'a> + 'static,
{
    let full_path = PathBuf::from(DOCS_DIR).join(path).with_extension("md");

    let md = fs::read_to_string(&full_path).expect("failed to read md file");
    mdsycx::parse(&md)
        .unwrap_or_else(|err| panic!("failed to parse md file `{}`: {err:?}", full_path.display()))
}

pub static DOCS: std::sync::LazyLock<HashMap<DocPage, ParseRes<DocFrontmatter>>> =
    LazyLock::new(|| {
        let mut docs = HashMap::new();

        // First parse all the top-level sections. Add any subsectiosn to a buffer to be parsed
        // later.
        let mut subsections = Vec::new();

        for section in SECTIONS_JSON.sections.iter() {
            let doc = parse_doc::<DocFrontmatter>(&format!("next/{section}"));
            // Add subsections to buffer.
            subsections.extend(
                doc.front_matter
                    .subsections
                    .iter()
                    .map(|s| (section.clone(), s.clone())),
            );
            docs.insert(DocPage::new(section.clone(), None), doc);
        }

        for (section, subsection) in subsections {
            let doc = parse_doc::<DocFrontmatter>(&format!("next/{section}/{subsection}"));
            // Subsections don't have subsections.
            if !doc.front_matter.subsections.is_empty() {
                panic!("Subsections cannot have subsections: `{section}/{subsection}`");
            } else {
                docs.insert(DocPage::new(section, Some(subsection)), doc);
            }
        }
        docs
    });

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

pub static BOOK_INDEX: LazyLock<BookIndex> = LazyLock::new(|| {
    let mut sections = vec![];

    for section in SECTIONS_JSON.sections.iter() {
        let path = DocPage::new(section.clone(), None);
        let doc = DOCS.get(&path).expect("failed to get doc");
        let title = doc.front_matter.title.clone();
        let subsections = doc
            .front_matter
            .subsections
            .iter()
            .map(|subsection| {
                let path = DocPage::new(section.clone(), Some(subsection.clone()));
                let doc = DOCS.get(&path).expect("failed to get doc");
                let title = doc.front_matter.title.clone();
                BookItem { title, path }
            })
            .collect();

        sections.push(BookSection {
            title,
            subsections,
            path,
        })
    }

    BookIndex { sections }
});

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

            let post = parse_doc::<PostFrontmatter>(&format!("posts/{name}"));

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

    for page in DOCS.keys() {
        match page.subsection() {
            Some(subsection) => paths.push((
                Routes::BookSubsection(
                    page.section().to_string(),
                    page.subsection().unwrap().to_string(),
                ),
                format!("/book/{}/{subsection}.html", page.section()),
            )),
            None => paths.push((
                Routes::BookSection(page.section().to_string()),
                format!("/book/{}.html", page.section()),
            )),
        }
    }

    paths
}

/// Generate an XML sitemap file.
pub fn generate_sitemap_xml() -> Result<String, std::fmt::Error> {
    static BASE_URL: &str = "https://sycamore-rs.netlify.app";

    let paths = get_static_paths();
    let mut buf = String::new();

    write!(
        &mut buf,
        r#"<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#
    )?;

    for (route, path) in paths {
        if route == Routes::NotFound {
            continue;
        }
        let path = path
            .strip_suffix(".html")
            .expect("should be an html page")
            .trim_end_matches("index");
        let loc = format!("{BASE_URL}{path}");

        write!(&mut buf, r#"<url><loc>{loc}</loc></url>"#)?;
    }

    write!(&mut buf, r#"</urlset>"#)?;

    Ok(buf)
}
