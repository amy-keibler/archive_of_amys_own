use std::time::Duration;

use reqwest::{Client, ClientBuilder};
use scraper::{Html, Selector};

const USER_AGENT: &str = "ArchiveOfOurOwnDownloader";
const TIMEOUT: Duration = Duration::from_secs(5 * 60);

const AO3_BASE_URL: &str = "https://archiveofourown.org";

pub struct ArchiveOfOurOwnDownloader {
    client: Client,
}

impl ArchiveOfOurOwnDownloader {
    pub fn new() -> Result<Self, ArchiveOfOurOwnDownloadError> {
        let client = ClientBuilder::new()
            .user_agent(USER_AGENT)
            .cookie_store(true)
            .timeout(TIMEOUT)
            .https_only(true)
            .build()
            .map_err(ArchiveOfOurOwnDownloadError::ClientBuildError)?;
        Ok(Self { client })
    }

    pub async fn download(&mut self, work_id: u32) -> Result<(), ArchiveOfOurOwnDownloadError> {
        let response = self
            .client
            .get(format!("{AO3_BASE_URL}/works/{work_id}/?view_adult=true"))
            .send()
            .await
            .map_err(|e| ArchiveOfOurOwnDownloadError::PageRequestError(work_id, e))?;

        if response.status().is_success() {
            let page_body = response
                .text()
                .await
                .map_err(|e| ArchiveOfOurOwnDownloadError::PageBodyRequestError(work_id, e))?;

            let download_url = find_epub_download_url(&page_body);
            dbg!(download_url);
        } else {
            todo!("Handle other responses");
        }
        todo!("Download")
    }
}

fn find_epub_download_url(page_body: &str) -> Result<String, ArchiveOfOurOwnDownloadError> {
    let document = Html::parse_document(page_body);
    let selector = Selector::parse(r#"li.download li > a[href*=".epub"]"#)
        .map_err(|e| ArchiveOfOurOwnDownloadError::InvalidCssStyleSelectorError(format!("{e}")))?;
    let anchor_node = document.select(&selector).next();
    if let Some(anchor_node) = anchor_node {
        anchor_node
            .value()
            .attr("href")
            .map(|url| format!("{AO3_BASE_URL}{url}"))
            .ok_or_else(|| ArchiveOfOurOwnDownloadError::FailedToFindDownloadUrlError)
    } else {
        Err(ArchiveOfOurOwnDownloadError::FailedToFindDownloadUrlError)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ArchiveOfOurOwnDownloadError {
    #[error("Could not build ArchiveOfOurOwnDownloader client: {0}")]
    ClientBuildError(reqwest::Error),

    #[error("Could not get the page for work with ID {0}: {1}")]
    PageRequestError(u32, reqwest::Error),

    #[error("Could not get the body of the page for work with ID {0}: {1}")]
    PageBodyRequestError(u32, reqwest::Error),

    #[error("Invalid CSS-style selector: {0}")]
    InvalidCssStyleSelectorError(String),

    #[error("Failed to find the download URL (AO3 might have changed their page HTML)")]
    FailedToFindDownloadUrlError,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FIC_ID: u32 = 36358546; // https://archiveofourown.org/works/36358546/

    #[tokio::test]
    async fn it_should_download() {
        let mut downloader = ArchiveOfOurOwnDownloader::new().expect("Failed to build downloader");
        downloader
            .download(EXAMPLE_FIC_ID)
            .await
            .expect("Failed to download fanfic epub");
    }
}
