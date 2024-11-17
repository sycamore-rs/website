//! Get stats from the GitHub and crates.io API.

use std::sync::LazyLock;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

static OWNER: &str = "sycamore-rs";
static REPO: &str = "sycamore";

static CRATE: &str = "sycamore";

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .expect("could not create reqwest client")
});

static CACHE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/target/api_stats_cache.json");

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub latest_release: LatestRelease,
    pub repo_stats: RepoStats,
    pub contributors: Vec<Contributor>,
    pub crates_io: CratesIo,
}

static CACHED_VALUES: LazyLock<Cache> = LazyLock::new(|| {
    // If file does not exist, create it first.
    if !std::path::Path::new(CACHE).exists() {
        let latest_release = format!("https://api.github.com/repos/{OWNER}/{REPO}/releases/latest");
        // TODO: This will no longer work if we get over 100 contributors.
        let contributors =
            format!("https://api.github.com/repos/{OWNER}/{REPO}/contributors?per_page=100");
        let repo_stats = format!("https://api.github.com/repos/{OWNER}/{REPO}");
        let crates_io = format!("https://crates.io/api/v1/crates/{CRATE}");

        let values = std::thread::spawn(move || {
            Ok::<Cache, reqwest::Error>(Cache {
                latest_release: CLIENT.get(&latest_release).send()?.json()?,
                contributors: CLIENT.get(&contributors).send()?.json()?,
                repo_stats: CLIENT.get(&repo_stats).send()?.json()?,
                crates_io: CLIENT.get(&crates_io).send()?.json()?,
            })
        })
        .join()
        .unwrap()
        .expect("could not get values from GitHub API");

        let file = std::fs::File::create(CACHE).expect("could not create cache file");
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CratesIo {
    #[serde(rename = "crate")]
    pub _crate: Crate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crate {
    pub downloads: u32,
}

pub fn get_crate_io_stats() -> CratesIo {
    CACHED_VALUES.crates_io.clone()
}
