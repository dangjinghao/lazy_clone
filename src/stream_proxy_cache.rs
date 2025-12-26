use actix_web::HttpResponse;
use async_stream;
use futures_util::StreamExt;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

pub struct StreamProxyCache {
    download_dir: PathBuf,
}

impl StreamProxyCache {
    pub fn new(download_dir: PathBuf) -> Self {
        Self { download_dir }
    }

    pub async fn read_from_cache(&self, url_path: &str) -> Option<Vec<u8>> {
        let filepath = self.download_dir.join(url_path);
        if filepath.exists() {
            tokio::fs::read(&filepath).await.ok()
        } else {
            None
        }
    }

    /// Downloads the content from target_url, saves it to cache, and streams it back as HttpResponse
    pub async fn stream_and_cache(
        &self,
        target_url: &str,
        url_path: &str,
    ) -> Result<HttpResponse, String> {
        // Make the request
        let client = reqwest::Client::new();
        let response = client
            .get(target_url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch: {}", e))?;

        let status = response.status();
        if !status.is_success() {
            return Ok(HttpResponse::build(
                actix_web::http::StatusCode::from_u16(status.as_u16()).unwrap(),
            )
            .finish());
        }

        // Prepare file path
        let filepath = self.download_dir.join(url_path);

        // Create directory
        if let Some(parent) = filepath.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("Failed to create dir: {}", e))?;
        }

        // Open file for writing
        let mut file = tokio::fs::File::create(&filepath)
            .await
            .map_err(|e| format!("Failed to create file: {}", e))?;

        println!("Streaming and saving {} to {:?}", target_url, filepath);

        // Get content-type
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();

        let mut stream = response.bytes_stream();

        let stream_with_save = async_stream::stream! {
            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        // Write to file
                        if let Err(e) = file.write_all(&chunk).await {
                            eprintln!("Failed to write to file: {}", e);
                            break;
                        }
                        // Send to client
                        yield Ok::<_, actix_web::Error>(chunk);
                    }
                    Err(e) => {
                        eprintln!("Stream error: {}", e);
                        break;
                    }
                }
            }
            // Make sure to flush the file at the end
            file.flush().await.ok();
        };

        Ok(HttpResponse::Ok()
            .content_type(content_type)
            .streaming(Box::pin(stream_with_save)))
    }

    pub fn normalize_path(parsed_url: &reqwest::Url) -> String {
        let mut path = String::from(parsed_url.path().trim_start_matches('/'));
        if path.is_empty() {
            path = "index.html".to_string();
        } else if path.ends_with('/') {
            path.push_str("index.html");
        }
        path
    }
}
