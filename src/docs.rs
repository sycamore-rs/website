//! Shared documentation source and URL model (also available in the browser).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocsSource {
    pub slug: &'static str,
    pub checkout: &'static str,
    pub branch: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/docs_sources.rs"));
// Generated from the same referenced Markdown pages used by static rendering.
include!(concat!(env!("OUT_DIR"), "/page_paths.rs"));

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocPage(pub String, pub Option<String>);

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

impl DocsSource {
    pub fn latest() -> Self {
        DOCS_SOURCES[0]
    }
    pub fn is_latest(self) -> bool {
        self.slug == "main"
    }
    pub fn label(self) -> &'static str {
        if self.is_latest() {
            "Latest (development)"
        } else {
            self.slug
        }
    }
    pub fn url(self, page: &DocPage) -> String {
        let prefix = if self.is_latest() {
            "/book".to_owned()
        } else {
            format!("/book/{}", self.slug)
        };
        match page.subsection() {
            Some(doc) => format!("{prefix}/{}/{doc}", page.section()),
            None => format!("{prefix}/{}", page.section()),
        }
    }
    pub fn component_id(self, page: &DocPage) -> String {
        // Delimit components unambiguously; slugs cannot contain '/'.
        format!(
            "Book/{}/{}",
            self.slug,
            match page.subsection() {
                Some(doc) => format!("{}/{doc}", page.section()),
                None => page.section().to_owned(),
            }
        )
    }
    pub fn edit_url(self, page: &DocPage) -> String {
        let path = match page.subsection() {
            Some(doc) => format!("{}/{doc}", page.section()),
            None => page.section().to_owned(),
        };
        format!(
            "https://github.com/sycamore-rs/sycamore/edit/{}/docs/next/{path}.md",
            self.branch
        )
    }
}

/// Decode the catch-all book route without ambiguous section/version routes.
pub fn resolve_book(path: &[String]) -> Option<(DocsSource, DocPage)> {
    let first = path.first()?;
    let (source, rest) = match DOCS_SOURCES
        .iter()
        .find(|s| !s.is_latest() && s.slug == first)
    {
        Some(source) => (*source, &path[1..]),
        None => (DocsSource::latest(), path),
    };
    match rest {
        [section] => Some((source, DocPage::new(section.clone(), None))),
        [section, doc] => Some((source, DocPage::new(section.clone(), Some(doc.clone())))),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn urls_and_routes_round_trip() {
        for source in DOCS_SOURCES {
            for page in [
                DocPage::new("introduction".into(), None),
                DocPage::new("introduction".into(), Some("installation".into())),
            ] {
                let url = source.url(&page);
                let segments = url
                    .trim_start_matches("/book/")
                    .split('/')
                    .map(str::to_owned)
                    .collect::<Vec<_>>();
                assert_eq!(resolve_book(&segments), Some((*source, page.clone())));
                if !source.is_latest() {
                    assert_ne!(
                        source.component_id(&page),
                        DocsSource::latest().component_id(&page)
                    );
                }
            }
        }
        assert!(resolve_book(&[]).is_none());
        assert!(resolve_book(&["a".into(), "b".into(), "c".into()]).is_none());
    }
}
