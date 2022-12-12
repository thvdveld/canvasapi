use anyhow::anyhow;
use std::marker::PhantomData;

use crate::canvas::CanvasInformation;
use crate::parameters::*;

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

/// An object for get requests with a single map.
pub struct GetObjectRequestMap<Output>
where
    Output: serde::de::DeserializeOwned + Clone + Copy + std::str::FromStr,
    <Output as std::str::FromStr>::Err: std::fmt::Debug,
{
    url: String,
    parameters: Vec<RequestParameter>,
    output: PhantomData<Output>,
    key: String,
}

impl<Output> GetObjectRequestMap<Output>
where
    Output: serde::de::DeserializeOwned + Clone + Copy + std::str::FromStr,
    <Output as std::str::FromStr>::Err: std::fmt::Debug,
{
    pub fn new(url: String, key: String) -> Self {
        Self {
            url,
            parameters: vec![],
            output: PhantomData,
            key,
        }
    }

    #[cfg(feature = "blocking")]
    pub fn fetch(
        mut self,
        canvas: &CanvasInformation<'_>,
    ) -> anyhow::Result<GetObjectResponse<Output>> {
        let resp: std::collections::HashMap<String, String> = canvas
            .get_request(canvas.add_url_prefix(&self.url))
            .send()?
            .json()?;

        let val: Output = resp[&self.key]
            .parse()
            .map_err(|_| anyhow!("Failed to parse output"))?;

        Ok(GetObjectResponse(val))
    }

    #[cfg(not(feature = "blocking"))]
    pub async fn fetch(
        mut self,
        canvas: &CanvasInformation<'_>,
    ) -> anyhow::Result<GetObjectResponse<Output>> {
        let resp: std::collections::HashMap<String, String> = canvas
            .get_request(canvas.add_url_prefix(&self.url))
            .send()
            .await?
            .json()
            .await?;

        let val: Output = resp[&self.key]
            .parse()
            .map_err(|_| anyhow!("Failed to parse output"))?;

        Ok(GetObjectResponse(val))
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

    #[cfg(feature = "blocking")]
    pub fn fetch(
        mut self,
        canvas: &CanvasInformation<'_>,
    ) -> anyhow::Result<GetObjectResponse<Output>> {
        let resp = canvas
            .get_request(canvas.add_url_prefix(&self.url))
            .send()?
            .json::<Output>()?;

        Ok(GetObjectResponse(resp))
    }

    #[cfg(not(feature = "blocking"))]
    pub async fn fetch(
        mut self,
        canvas: &CanvasInformation<'_>,
    ) -> anyhow::Result<GetObjectResponse<Output>> {
        let resp = canvas
            .get_request(canvas.add_url_prefix(&self.url))
            .send()
            .await?
            .json::<Output>()
            .await?;

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

    #[cfg(feature = "blocking")]
    pub fn fetch(
        mut self,
        canvas: &CanvasInformation<'_>,
    ) -> anyhow::Result<GetObjectResponse<Vec<Output>>> {
        let mut output: Vec<Output> = vec![];
        let mut url: String = canvas.add_url_prefix(&self.url);

        if self.parameters.len() > 0 {
            url = format!("{}?", url);
            let mut first = true;
            for parameter in &self.parameters {
                if first {
                    let name = &parameter.name;
                    let value = &parameter.value;
                    url = format!("{url}{name}={value}");
                    first = false;
                } else {
                    let name = &parameter.name;
                    let value = &parameter.value;
                    url = format!("{url}&{name}={value}");
                }
            }
        }

        loop {
            let mut client = canvas.get_request(url);

            let mut resp = client.send()?;
            let headers = resp.headers().clone();

            let next_url = get_next_url(&headers)?;

            output.extend(resp.json::<Vec<Output>>()?);

            url = if next_url.is_none() {
                break;
            } else {
                next_url.unwrap().to_string()
            };
        }

        Ok(GetObjectResponse(output))
    }

    #[cfg(not(feature = "blocking"))]
    pub async fn fetch(
        mut self,
        canvas: &CanvasInformation<'_>,
    ) -> anyhow::Result<GetObjectResponse<Vec<Output>>> {
        use anyhow::bail;
        use reqwest::StatusCode;

        let mut output: Vec<Output> = vec![];
        let mut url: String = canvas.add_url_prefix(&self.url);

        if !self.parameters.is_empty() {
            url = format!("{}?", url);
            let mut first = true;
            for parameter in &self.parameters {
                if first {
                    let name = &parameter.name;
                    let value = &parameter.value;
                    url = format!("{url}{name}={value}");
                    first = false;
                } else {
                    let name = &parameter.name;
                    let value = &parameter.value;
                    url = format!("{url}&{name}={value}");
                }
            }
        }

        loop {
            let mut client = canvas.get_request(url);

            let mut resp = client.send().await?;

            match resp.status() {
                StatusCode::OK => {},
                status => bail!("{status}"),
            }

            let headers = resp.headers().clone();

            let next_url = get_next_url(&headers)?;

            output.extend(resp.json::<Vec<Output>>().await?);

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
fn get_next_url(resp: &reqwest::header::HeaderMap) -> anyhow::Result<Option<&str>> {
    let headers = resp.get("link");
    if let Some(headers) = headers {
        let headers = headers.to_str()?;
        let headers: Vec<&str> = headers.split(',').map(|x| x.trim()).collect();

        for header in headers {
            if header.contains("next") {
                return Ok(Some(
                    header
                        .split(';')
                        .map(|x| &x[1..x.len() - 1])
                        .collect::<Vec<&str>>()[0],
                ));
            }
        }

        Ok(None)
    } else {
        Ok(None)
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

macro_rules! feature {
    ($feature_name:ident) => {
        /// This function depends on the `$feature_name` feature.
        #[cfg(feature = "$feature_name")]
    };
}

macro_rules! api_get {
    (
        $(#[$outer:meta])*
        $name:ident ($($self:ident)?):
            $path:expr =>
            ($($named_self_arg:ident : $named_self_val:expr),* $(,)?)
            -> ($($path_val:ident: $path_ty:ty),*)
            -> $ret_ty:ident
            $([$($param_val:expr),*])?
            $(features = [$($feature_name:expr),+])?
    ) => {
        $(#[$outer])*
        $(
            $(
            #[doc = "\nThe `"]
            #[doc = $feature_name]
            #[doc = "` feature is needed to use this function."]
            #[cfg(feature = $feature_name)]
            )*
        )?
        pub fn $name($(&$self,)? $($path_val:$path_ty,)*) -> anyhow::Result<GetObjectRequest<$ret_ty>> {
            Ok(GetObjectRequest::<$ret_ty>::new(
                format!($path
                    $(,$named_self_arg=$named_self_val)*
                    $(,$path_val=$path_val)*))
            $($(.add_parameter($param_val))*)?)
        }
    };

    (
        $(#[$outer:meta])*
        $name:ident ($($self:ident)?):
            $path:expr =>
            ($($named_self_arg:ident : $named_self_val:expr),* $(,)?)
            -> ($($path_val:ident: $path_ty:ty),*)
            -> {$ret_name_field:literal :$ret_ty:ty}
            $([$($param_val:expr),*])?
            $(features = [$($feature_name:expr),+])?
    ) => {
        $(#[$outer])*
        $(
            $(
            #[doc = "\nThe `"]
            #[doc = $feature_name]
            #[doc = "` feature is needed to use this function."]
            #[cfg(feature = $feature_name)]
            )*
        )?
        pub fn $name($(&$self,)? $($path_val:$path_ty,)*) -> anyhow::Result<GetObjectRequestMap<$ret_ty>> {
            Ok(GetObjectRequestMap::<$ret_ty>::new(
                format!($path
                    $(,$named_self_arg=$named_self_val)*
                    $(,$path_val=$path_val)*), $ret_name_field.into())
            $($(.add_parameter($param_val))*)?)
        }
    };

    (
        $(#[$outer:meta])*
        $name:ident ($($self:ident)?):
            $path:expr =>
            ($($named_self_arg:ident : $named_self_val:expr),* $(,)?)
            -> ($($path_val:ident: $path_ty:ty),*)
            -> [$ret_ty:ty]
            $([$($param_val:expr),*])?
            $(features = [$(( name = $feature_name:expr, reason = $feature_reason:expr )),+])?
    ) => {
        $(#[$outer])*
        $(
            $(
            #[doc = "\nThe `"]
            #[doc = $feature_name]
            #[doc = "` feature is needed to use this function."]
            #[doc = $feature_reason]
            #[cfg(feature = $feature_name)]
            )*
        )?
        pub fn $name($(&$self,)? $($path_val:$path_ty,)*) -> anyhow::Result<GetPagedObjectRequest<$ret_ty>> {
            Ok(GetPagedObjectRequest::<$ret_ty>::new(
                format!($path
                    $(,$named_self_arg=$named_self_val)*
                    $(,$path_val=$path_val)*))
            $($(.add_parameter($param_val))*)?)
        }
    };
}
