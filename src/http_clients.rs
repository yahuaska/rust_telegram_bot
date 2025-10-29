use crate::api_client::HttpClient;
use std::future::Future;

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

impl HttpClient for ReqwestHttpClient {
    type Error = reqwest::Error;

    fn get<'a>(
        &'a self,
        url: &'a str,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send + 'a {
        async move {
            let response = reqwest::get(url).await?;
            let body = response.text().await?;
            Ok(body)
        }
    }

    fn post<'a>(
        &'a self,
        url: &'a str,
        body: &'a str,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send + 'a {
        async move {
            let response = self
                .client
                .post(url)
                .body(String::from(body))
                .send()
                .await?;
            let body = response.text().await?;
            Ok(body)
        }
    }

    fn format_error(&self, err: reqwest::Error) -> String {
        format!("{:#?}", err)
    }
}
