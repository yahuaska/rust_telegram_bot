use crate::http_client::HttpClient;
use reqwest::multipart;
use std::{collections::HashMap, path::PathBuf};

#[derive(Clone)]
pub struct ReqwestHttpClient {
    client: reqwest::Client,
}

impl ReqwestHttpClient {
    pub fn new() -> Self {
        Self {
            client: match reqwest::Client::builder().build() {
                Ok(client) => client,
                Err(err) => panic!("Failed to create reqwest client: {}", err),
            },
        }
    }
}

impl Default for ReqwestHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClient for ReqwestHttpClient {
    type Error = reqwest::Error;

    async fn get(&self, url: &str) -> Result<String, Self::Error> {
        let response = reqwest::get(url).await?;
        let body = response.text().await?;
        Ok(body)
    }

    async fn post_multipart(
        &self,
        url: &str,
        body: HashMap<String, String>,
        file: Option<&str>,
    ) -> Result<String, Self::Error> {
        let mut form = multipart::Form::new();
        for (k, v) in body {
            let part = multipart::Part::text(v).mime_str("text/plain")?;
            form = form.part(k, part);
        }
        let form = match file {
            Some(file) => {
                let file_name = PathBuf::from(file)
                    .file_name()
                    .and_then(|x| x.to_str())
                    .unwrap_or(file)
                    .to_string();
                println!("file_path: {file}");
                if let Ok(file_part) = multipart::Part::file(file).await {
                    let file_part = file_part.file_name("test.mp4");
                    form.part(file_name, file_part)
                } else {
                    form
                }
            }
            None => form,
        };
        println!("Form: {form:#?}");
        let request = self
            .client
            .post(url)
            .header("Content-Type", "multipart/form-data")
            .multipart(form);
        println!("Request: {request:#?}");
        let response = request.send().await?;
        let body = response.text().await?;
        println!("{body}");
        Ok(body)
    }

    async fn post(&self, url: &str, body: String) -> Result<String, Self::Error> {
        let request = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body);
        println!("Request: {request:#?}");
        let response = request.send().await?;
        let body = response.text().await?;
        println!("{body}");
        Ok(body)
    }

    fn format_error(&self, err: reqwest::Error) -> String {
        format!("{:#?}", err)
    }
}
