use crate::canvas::CanvasInformation;
use crate::parameters::*;

use std::marker::PhantomData;

pub struct GetObjectResponse<Output>(Output)
where
    Output: serde::de::DeserializeOwned;

impl<Output> GetObjectResponse<Output>
where
    Output: serde::de::DeserializeOwned,
{
    pub fn inner(self) -> Output {
        self.0
    }
}

/// An object for get requests.
pub struct GetObjectRequest<Output>
where
    Output: serde::de::DeserializeOwned,
{
    url: String,
    parameters: Vec<RequestParameter>,
    output: PhantomData<Output>,
}

impl<Output> GetObjectRequest<Output>
where
    Output: serde::de::DeserializeOwned,
{
    /// Create a new get request object with an correct url.
    pub fn new(url: String) -> Self {
        Self {
            url,
            parameters: vec![],
            output: PhantomData,
        }
    }

    /// Do a get request.
    pub async fn fetch(
        mut self,
        canvas: &CanvasInformation,
    ) -> Result<GetObjectResponse<Output>, anyhow::Error> {
        let resp = canvas
            .get_request(canvas.add_url_prefix(&self.url))
            .send()
            .await
            .unwrap()
            .json::<Output>()
            .await
            .unwrap();

        Ok(GetObjectResponse(resp))
    }
}

pub struct GetPagedObjectRequest<Output>
where
    Output: serde::de::DeserializeOwned,
{
    url: String,
    parameters: Vec<RequestParameter>,
    output: PhantomData<Vec<Output>>,
}

impl<Output> GetPagedObjectRequest<Output>
where
    Output: serde::de::DeserializeOwned + std::fmt::Debug,
{
    pub fn new(url: String) -> Self {
        Self {
            url,
            parameters: vec![],
            output: PhantomData,
        }
    }

    pub async fn fetch(
        mut self,
        canvas: &CanvasInformation,
    ) -> Result<GetObjectResponse<Vec<Output>>, anyhow::Error> {
        let mut output: Vec<Output> = vec![];
        let mut url: String = canvas.add_url_prefix(&self.url);

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
            let mut client = canvas.get_request(url);

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

        Ok(GetObjectResponse(output))
    }

    pub fn add_parameter(mut self, parameter: impl Into<RequestParameter>) -> Self {
        self.parameters.push(parameter.into());
        self
    }
}

/// Get the next url for paging from the header information.
fn get_next_url<'i>(resp: &'i reqwest::header::HeaderMap) -> Option<&'i str> {
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

macro_rules! api_todo {
    (
        $(#[$outer:meta])*
        $name:ident($($self:ident)?)
    ) => {
        $(#[$outer])*
        #[doc(hidden)]
        pub fn $name($(&$self)?) {
            unimplemented!();
        }
    };
}

macro_rules! api_get {
    (
        $(#[$outer:meta])*
        $name:ident ($($self:ident)?):
            $path:expr =>
            ($($named_self_arg:ident : $named_self_val:expr),*)
            -> ($($path_val:ident: $path_ty:ty),*)
            -> $ret_ty:ident
            $([$($param_val:expr),*])?
    ) => {
        $(#[$outer])*
        pub fn $name($(&$self,)? $($path_val:$path_ty,)*) -> GetObjectRequest<$ret_ty> {
            GetObjectRequest::<$ret_ty>::new(
                format!($path
                    $(,$named_self_arg=$named_self_val)*
                    $(,$path_val=$path_val)*))
            $($(.add_parameter($param_val))*)?
        }
    };

    (
        $(#[$outer:meta])*
        $name:ident ($($self:ident)?):
            $path:expr =>
            ($($named_self_arg:ident : $named_self_val:expr),*)
            -> ($($path_val:ident: $path_ty:ty),*)
            -> [$ret_ty:ident]
            $([$($param_val:expr),*])?
    ) => {
        $(#[$outer])*
        pub fn $name($(&$self,)? $($path_val:$path_ty,)*) -> GetPagedObjectRequest<$ret_ty> {
            GetPagedObjectRequest::<$ret_ty>::new(
                format!($path
                    $(,$named_self_arg=$named_self_val)*
                    $(,$path_val=$path_val)*))
            $($(.add_parameter($param_val))*)?
        }
    };
}
