use crate::api_client::HttpClient;

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

    async fn post(&self, url: &str, body: &str) -> Result<String, Self::Error> {
        let response = self
            .client
            .post(url)
            .body(String::from(body))
            .send()
            .await?;
        let body = response.text().await?;
        Ok(body)
    }

    fn format_error(&self, err: reqwest::Error) -> String {
        format!("{:#?}", err)
    }
}
