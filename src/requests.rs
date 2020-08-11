use crate::canvas::Canvas;
use crate::parameters::*;

/// An object for get requests.
pub struct GetObjectRequest<'i, Output>
where
    Output: serde::de::DeserializeOwned,
{
    url: String,
    canvas: &'i Canvas,
    parameters: Vec<RequestParameter>,
    inner: Option<Output>,
}

impl<'i, Output> GetObjectRequest<'i, Output>
where
    Output: serde::de::DeserializeOwned,
{
    /// Create a new get request object with an correct url.
    pub fn new(url: String, canvas: &'i Canvas) -> Self {
        Self {
            url,
            canvas,
            parameters: vec![],
            inner: None,
        }
    }

    /// Do a get request.
    pub async fn fetch(
        mut self,
    ) -> Result<GetObjectRequest<'i, Output>, Box<dyn std::error::Error>> {
        let resp = self
            .canvas
            .get_request(self.canvas.add_url_prefix(&self.url))
            .send()
            .await
            .unwrap()
            .json::<Output>()
            .await
            .unwrap();

        self.inner = Some(resp);
        Ok(self)
    }

    /// Get the result.
    pub fn inner(self) -> Option<Output> {
        self.inner
    }
}

pub struct GetPagedObjectRequest<'i, Output>
where
    Output: serde::de::DeserializeOwned,
{
    url: String,
    canvas: &'i Canvas,
    parameters: Vec<RequestParameter>,
    inner: Option<Vec<Output>>,
}

impl<'i, Output> GetPagedObjectRequest<'i, Output>
where
    Output: serde::de::DeserializeOwned + std::fmt::Debug,
{
    pub fn new(url: String, canvas: &'i Canvas) -> Self {
        Self {
            url,
            canvas,
            parameters: vec![],
            inner: None,
        }
    }

    pub async fn fetch(
        mut self,
    ) -> Result<GetPagedObjectRequest<'i, Output>, Box<dyn std::error::Error>> {
        let mut output: Vec<Output> = vec![];
        let mut url: String = self.canvas.add_url_prefix(&self.url);

        if self.parameters.len() > 0 {
            url = format!("{}?", url);
            let mut first = true;
            for parameter in &self.parameters {
                if first {
                    url = format!("{}{}={}", url, parameter.name, parameter.value);
                    first = false;
                } else {
                    url = format!("{}&{}={}", url, parameter.name, parameter.value);
                }
            }
        }

        loop {
            let mut client = self.canvas.get_request(url);

            let mut resp = client.send().await.unwrap();
            let headers = resp.headers().clone();

            let next_url = get_next_url(&headers);

            output.extend(resp.json::<Vec<Output>>().await.unwrap());

            url = if next_url.is_none() {
                break;
            } else {
                next_url.unwrap().to_string()
            };
        }

        self.inner = Some(output);

        Ok(self)
    }

    pub fn add_parameter(mut self, parameter: impl Into<RequestParameter>) -> Self {
        self.parameters.push(parameter.into());
        self
    }

    pub fn inner(self) -> Option<Vec<Output>> {
        self.inner
    }
}

/// Get the next url for paging from the header information.
fn get_next_url<'i>(resp: &'i actix_web::http::HeaderMap) -> Option<&'i str> {
    let headers = resp.get("link");
    if headers.is_none() {
        None
    } else {
        let headers = headers.unwrap().to_str().unwrap();
        let headers: Vec<&str> = headers.split(",").map(|x| x.trim()).collect();

        for header in headers {
            if header.contains("next") {
                return Some(
                    header
                        .split(";")
                        .map(|x| &x[1..x.len() - 1])
                        .collect::<Vec<&str>>()[0],
                );
            }
        }

        None
    }
}
