//! Get stats from GitHub api.

use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

static OWNER: &str = "sycamore-rs";
static REPO: &str = "sycamore";

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

static CLIENT: LazyLock<reqwest::blocking::Client> = LazyLock::new(|| {
    reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .expect("could not create reqwest client")
});

static CACHE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/target/github_stats_cache.json"
);

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub latest_release: LatestRelease,
    pub repo_stats: RepoStats,
    pub contributors: Vec<Contributor>,
}

static CACHED_VALUES: LazyLock<Cache> = LazyLock::new(|| {
    // If file does not exist, create it first.
    if !std::path::Path::new(CACHE).exists() {
        let file = std::fs::File::create(CACHE).expect("could not create cache file");

        let latest_release = format!("https://api.github.com/repos/{OWNER}/{REPO}/releases/latest");
        let contributors = format!("https://api.github.com/repos/{OWNER}/{REPO}/contributors");
        let repo_stats = format!("https://api.github.com/repos/{OWNER}/{REPO}");

        let values = std::thread::spawn(move || {
            Ok::<Cache, reqwest::Error>(Cache {
                latest_release: CLIENT.get(&latest_release).send()?.json()?,
                contributors: CLIENT.get(&contributors).send()?.json()?,
                repo_stats: CLIENT.get(&repo_stats).send()?.json()?,
            })
        })
        .join()
        .unwrap()
        .expect("could not get values from GitHub API");
        serde_json::to_writer(file, &values).expect("could not write to cache file");
    }
    let file = std::fs::File::open(CACHE).expect("could not open cache file");
    serde_json::from_reader(file).expect("could not parse cache file")
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestRelease {
    pub html_url: String,
    pub name: String,
}

/// Get the latest release name from the GitHub API.
pub fn get_latest_release() -> LatestRelease {
    CACHED_VALUES.latest_release.clone()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoStats {
    pub stargazers_count: u32,
}

pub fn get_repo_stats() -> RepoStats {
    CACHED_VALUES.repo_stats.clone()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contributor {
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
}

pub fn get_contributors() -> Vec<Contributor> {
    CACHED_VALUES.contributors.clone()
}
