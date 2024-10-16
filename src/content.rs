use mdsycx::ParseRes;

use std::{collections::HashMap, fs, sync::LazyLock};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PostFrontmatter {
    pub title: String,
    pub description: String,
    pub date: String,
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
