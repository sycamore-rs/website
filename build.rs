use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn git_config(args: &[&str]) -> String {
    let output = Command::new("git")
        .args(["config", "-f", ".gitmodules"])
        .args(args)
        .output()
        .expect("git is required to read documentation checkout configuration");
    assert!(
        output.status.success(),
        "failed to read .gitmodules: {args:?}"
    );
    String::from_utf8(output.stdout).unwrap()
}

fn main() {
    println!("cargo:rerun-if-changed=.gitmodules");
    println!("cargo:rerun-if-env-changed=DOCS_DIR");
    let paths = git_config(&["--get-regexp", r"^submodule\..*\.path$"]);
    let mut sources = Vec::new();
    for line in paths.lines() {
        let (key, path) = line.split_once(' ').expect("invalid submodule path");
        let Some(slug) = path.strip_prefix("checkouts/") else {
            continue;
        };
        assert!(
            !slug.is_empty()
                && slug
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || "-._".contains(c)),
            "checkout names must be URL-safe: {slug}"
        );
        let branch_key = format!("{}.branch", key.strip_suffix(".path").unwrap());
        let branch = git_config(&["--get", &branch_key]);
        sources.push((slug.to_owned(), path.to_owned(), branch.trim().to_owned()));
    }
    assert!(
        sources.iter().any(|(slug, _, _)| slug == "main"),
        "checkouts/main is required"
    );
    sources.sort_by(|a, b| {
        (a.0 != "main")
            .cmp(&(b.0 != "main"))
            .then_with(|| b.0.cmp(&a.0))
    });
    let entries: String = sources
        .iter()
        .map(|(slug, path, branch)| {
            format!("DocsSource {{ slug: {slug:?}, checkout: {path:?}, branch: {branch:?} }},\n")
        })
        .collect();
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("docs_sources.rs"),
        format!("pub const DOCS_SOURCES: &[DocsSource] = &[{entries}];\n"),
    )
    .unwrap();
    write_page_manifest(&sources);
}

/// Mirror the book loader's section/subsection selection, rather than treating
/// every Markdown file (or every syntactically valid URL) as a published page.
fn write_page_manifest(sources: &[(String, String, String)]) {
    let latest_docs = env::var("DOCS_DIR").unwrap_or_else(|_| "checkouts/main/docs".into());
    let mut pages = vec!["/".to_owned()];
    for (slug, checkout, _) in sources {
        let docs = if slug == "main" {
            PathBuf::from(&latest_docs)
        } else {
            PathBuf::from(checkout).join("docs")
        };
        let root = docs.join("next");
        println!("cargo:rerun-if-changed={}", root.display());
        let sections: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(root.join("sections.json")).expect("failed to read sections.json"),
        )
        .expect("invalid sections.json");
        let prefix = if slug == "main" {
            "/book".to_owned()
        } else {
            format!("/book/{slug}")
        };
        for section in sections["sections"]
            .as_array()
            .expect("sections must be an array")
        {
            let section = section.as_str().expect("section must be a string");
            pages.push(format!("{prefix}/{section}"));
            let path = root.join(section).with_extension("md");
            let md = fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
            let (_, rest) = md.trim().split_once("---").expect("missing front matter");
            let (frontmatter, _) = rest
                .split_once("---")
                .expect("missing front matter terminator");
            let frontmatter: yaml_serde::Value =
                yaml_serde::from_str(frontmatter).expect("invalid front matter");
            if let Some(subsections) = frontmatter.get("subsections") {
                for subsection in subsections
                    .as_sequence()
                    .expect("subsections must be an array")
                {
                    let subsection = subsection.as_str().expect("subsection must be a string");
                    assert!(
                        root.join(section)
                            .join(subsection)
                            .with_extension("md")
                            .is_file(),
                        "missing subsection: {section}/{subsection}"
                    );
                    pages.push(format!("{prefix}/{section}/{subsection}"));
                }
            }
        }
    }
    let posts = Path::new(&latest_docs).join("posts");
    println!("cargo:rerun-if-changed={}", posts.display());
    for entry in fs::read_dir(posts).expect("failed to read posts") {
        let path = entry.expect("failed to read post entry").path();
        if path.extension().is_some_and(|ext| ext == "md") {
            pages.push(format!(
                "/post/{}",
                path.file_stem().unwrap().to_str().unwrap()
            ));
        }
    }
    pages.sort();
    pages.dedup();
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("page_paths.rs"),
        format!("pub const PAGE_PATHS: &[&str] = &{pages:?};\n"),
    )
    .unwrap();
}
