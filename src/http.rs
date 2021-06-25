// This is an interface to an external HTTP library to make it easier to switch in the
// future, if needed.

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use url::Url;

use super::ErrorKind;

impl From<surf::Error> for ErrorKind {
    fn from(err: surf::Error) -> ErrorKind {
        Self::Api(err.to_string())
    }
}

pub struct Http {
    client: surf::Client,
}

#[derive(Deserialize)]
struct Message {
    #[serde(rename = "Message")]
    message: String,
}

impl Http {
    pub fn new() -> Self {
        Self {
            client: surf::Client::new(),
        }
    }

    pub async fn get<T>(&self, url: &Url) -> Result<T, ErrorKind>
    where
        //T: for<'de> Deserialize<'de>,
        T: DeserializeOwned,
    {
        let mut response = self.client.get(url).send().await?;
        match response.status() {
            surf::StatusCode::Ok => Ok(response.body_json().await?),
            code @ _ => {
                let Message { message } = response.body_json().await?;
                Err(ErrorKind::Http(code as u16, message))
            }
        }
    }

    pub async fn post<U, T>(&self, url: &Url, body: &U) -> Result<T, ErrorKind>
    where
        U: Serialize,
        T: for<'de> Deserialize<'de>,
    {
        let mut response = self
            .client
            .post(url)
            .body(serde_json::to_string(body).unwrap())
            .send()
            .await?;
        match response.status() {
            surf::StatusCode::Ok => Ok(response.body_json().await?),
            code @ _ => {
                let Message { message } = response.body_json().await?;
                Err(ErrorKind::Http(code as u16, message))
            }
        }
    }
}
