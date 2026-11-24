#[allow(unused_imports)]
use crate::prelude::*;

impl Release {
    /// Reads the release out of the address `releases/latest` redirects to.
    ///
    /// Asking for the redirect rather than the API costs one request, needs
    /// no token and is not rate limited the way the API is. The address ends
    /// in the tag: `https://github.com/owner/caret/releases/tag/v0.1.1`.
    pub fn from_redirect(url: &str) -> Option<Release> {
        let tag = url.trim().trim_end_matches('/').rsplit('/').next()?;
        if tag.is_empty() || !url.contains("/releases/tag/") {
            return None;
        }
        Some(Release {
            version: Version::parse(tag)?,
            tag: tag.to_string(),
        })
    }

    /// Where this release's files live.
    pub fn download_url(&self, file: &str) -> String {
        format!("{}/download/{}/{file}", crate::RELEASES, self.tag)
    }
}
