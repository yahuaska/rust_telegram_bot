use std::collections::HashMap;
use std::future::Future;

pub trait HttpClient {
    type Error;

    fn get<'a>(
        &'a self,
        url: &'a str,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send + 'a;
    fn post<'a>(
        &'a self,
        url: &'a str,
        body: String,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send + 'a;
    fn post_multipart<'a>(
        &'a self,
        url: &'a str,
        body: HashMap<String, String>,
        file: Option<&'a str>,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send + 'a;
    fn format_error(&self, error: Self::Error) -> String;
}
