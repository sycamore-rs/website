use mdsycx::ParseRes;
use serde::Deserialize;

use std::{collections::HashMap, fmt::Display, fs, str::FromStr, sync::LazyLock};

#[derive(Debug, Clone, Deserialize)]
pub struct PostFrontmatter {
    pub title: String,
    pub description: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub date: PostDate,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PostDate {
    pub day: u32,
    pub month: u32,
    pub year: u32,
}

impl PartialOrd for PostDate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.year, self.month, self.day).cmp(&(other.year, other.month, other.day)))
    }
}

impl Ord for PostDate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.year, self.month, self.day).cmp(&(other.year, other.month, other.day))
    }
}

impl FromStr for PostDate {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-').map(|x| x.parse().unwrap());
        Ok(PostDate {
            year: parts.next().ok_or("could not parse year")?,
            month: parts.next().ok_or("could not parse month")?,
            day: parts.next().ok_or("could not parse date")?,
        })
    }
}

/// Deserialize date in format "YYYY-MM-DD"
fn deserialize_date<'de, D>(deserializer: D) -> Result<PostDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // TODO: error handling
    Ok(PostDate::from_str(&s).unwrap())
}

impl Display for PostDate {
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

pub fn get_static_paths() -> Vec<String> {
    let mut paths = vec![];

    paths.push("/index.html".to_string());
    paths.push("/404.html".to_string());

    for post in POSTS.keys() {
        paths.push(format!("/post/{post}.html"));
    }

    paths
}
