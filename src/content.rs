use mdsycx::ParseRes;
use serde::Deserialize;

use std::{collections::HashMap, fmt::Display, fs, str::FromStr, sync::LazyLock};

#[derive(Debug, Clone, Deserialize)]
pub struct PostFrontmatter {
    pub title: String,
    pub description: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub date: Date,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DocFrontmatter {
    pub title: Option<String>,
}

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

pub static POSTS: std::sync::LazyLock<HashMap<String, ParseRes<PostFrontmatter>>> =
    LazyLock::new(|| {
        let mut posts = HashMap::new();

        for entry in fs::read_dir("sycamore/docs/posts").expect("failed to read posts directory") {
            let entry = entry.expect("failed to read post entry");
            let path = entry.path();
            let id = path
                .file_stem()
                .expect("failed to get file stem")
                .to_string_lossy();
            let md = fs::read_to_string(&path).expect("failed to read post file");

            let parsed = mdsycx::parse(&md).expect("failed to parse post file");

            posts.insert(id.to_string(), parsed);
        }

        posts
    });

#[allow(clippy::type_complexity)]
pub static DOCS: std::sync::LazyLock<HashMap<(String, Option<String>), ParseRes<DocFrontmatter>>> =
    LazyLock::new(|| {
        let mut docs = HashMap::new();

        for entry in fs::read_dir("sycamore/docs/next").expect("failed to read docs directory") {
            let entry = entry.expect("failed to read docs entry");
            let section = entry.path();
            let section_name = section
                .file_stem()
                .expect("failed to get doc section name")
                .to_string_lossy();
            if section.is_dir() {
                // Get sub-section docs.
                for entry in fs::read_dir(&section).expect("failed to read docs directory") {
                    let entry = entry.expect("failed to read docs entry");
                    let doc = entry.path();
                    let doc_name = doc
                        .file_stem()
                        .expect("failed to get doc name")
                        .to_string_lossy();
                    let md = fs::read_to_string(&doc).expect("failed to read docs file");

                    let parsed = mdsycx::parse(&md).expect("failed to parse docs file");

                    docs.insert(
                        (section_name.to_string(), Some(doc_name.to_string())),
                        parsed,
                    );
                }
            } else {
                // Get section doc.
                let md = fs::read_to_string(&section).expect("failed to read docs file");

                let parsed = mdsycx::parse(&md).expect("failed to parse docs file");

                docs.insert((section_name.to_string(), None), parsed);
            }
        }

        docs
    });

#[derive(Debug, Clone, Deserialize)]
pub struct BookSidebar {
    pub sections: Vec<BookSection>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookSection {
    pub title: String,
    pub items: Vec<BookDoc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookDoc {
    pub name: String,
    pub href: String,
}

pub static BOOK_SIDEBAR: LazyLock<BookSidebar> = LazyLock::new(|| {
    let sidebar_json =
        fs::read_to_string("sycamore/docs/next/sidebar.json").expect("failed to read sidebar.json");
    serde_json::from_str(&sidebar_json).expect("failed to parse sidebar.json")
});

pub fn get_static_paths() -> Vec<String> {
    let mut paths = vec![];

    paths.push("/index.html".to_string());
    paths.push("/404.html".to_string());

    for post in POSTS.keys() {
        paths.push(format!("/post/{post}"));
    }

    for (section, doc) in DOCS.keys() {
        match doc {
            Some(doc) => paths.push(format!("/book/{section}/{doc}")),
            None => paths.push(format!("/book/{section}")),
        }
    }

    paths
}
