use std::sync::Arc;

use reqwest::{Client, Error, IntoUrl, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::debug;

use crate::Auth;

#[derive(Copy, Clone, Debug)]
pub(crate) enum RequestMethod {
    Get,
    Put,
    Delete,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Error occurred during request {0:?}")]
    ReqwestError(String),
    #[error("Invalid token (Expired access tokens, Invalid access tokens, etc.)")]
    InvalidToken,
    #[error("Invalid Parameters")]
    InvalidParameters,
    #[error("Access is forbidden (DoS detected etc.)")]
    Forbidden,
    #[error("Not found")]
    NotFound,
    #[error("Status code : {0:?}")]
    StatusCode(StatusCode),
    #[error("{0}")]
    ParseError(#[from] serde_json::Error),
    #[error("access token missing")]
    AccessTokenError,
    #[error("{status} - {error}: {message}")]
    ErrorMessage {
        status: StatusCode,
        error: String,
        message: String,
    },
    #[error("{0}")]
    QuerySerError(#[from] serde_qs::Error),
}

impl From<reqwest::Error> for ApiError {
    fn from(e: Error) -> Self {
        Self::ReqwestError(format!("{:?}", e))
    }
}

#[derive(Debug, Clone, Deserialize)]
struct ApiRequestError {
    error: String,
    message: String,
}

#[derive(Clone, Debug)]
pub(crate) struct ApiRequest {
    auth: Arc<Auth>,
    http: reqwest::Client,
}

impl ApiRequest {
    pub fn new(auth: Arc<Auth>, http: Client) -> Self {
        Self { auth, http }
    }

    pub async fn get<D>(&self, url: impl IntoUrl, is_auth: bool) -> Result<D, ApiError>
    where
        D: DeserializeOwned,
    {
        self.api_request(url.into_url()?, RequestMethod::Get, None::<()>, is_auth)
            .await
    }

    pub async fn delete<D>(&self, url: impl IntoUrl, is_auth: bool) -> Result<D, ApiError>
    where
        D: DeserializeOwned,
    {
        self.api_request(url.into_url()?, RequestMethod::Delete, None::<()>, is_auth)
            .await
    }

    pub async fn put<D, P: Serialize>(
        &self,
        url: impl IntoUrl,
        data: Option<P>,
        is_auth: bool,
    ) -> Result<D, ApiError>
    where
        D: DeserializeOwned,
    {
        self.api_request(url.into_url()?, RequestMethod::Put, data, is_auth)
            .await
    }

    /// is_auth : Use user authentication in request; otherwise use ClientID header
    async fn api_request<D, P: Serialize>(
        &self,
        url: impl IntoUrl,
        method: RequestMethod,
        data: Option<P>,
        // whether to use oauth2 access token or client id header
        is_auth: bool,
    ) -> Result<D, ApiError>
    where
        D: DeserializeOwned,
    {
        let mut request = match method {
            RequestMethod::Get => self.http.get(url.into_url()?),
            RequestMethod::Delete => self.http.delete(url.into_url()?),
            RequestMethod::Put => self.http.put(url.into_url()?),
        };

        if matches!(method, RequestMethod::Put) {
            if let Some(data) = &data {
                request = request.form(data);
            }
        }

        let request = if is_auth {
            request.bearer_auth(self.auth.access_token().secret())
        } else {
            request.header("X-MAL-CLIENT-ID", &*self.auth.client_id())
        };

        let response = request.send().await?;

        let status = response.status();
        let text = response.text().await?;

        debug!(status = status.as_u16(), response = text, "mal reponse");

        let error = serde_json::from_str::<ApiRequestError>(&text);

        match status {
            StatusCode::BAD_REQUEST => {
                return Err(ApiError::InvalidParameters);
            }

            StatusCode::UNAUTHORIZED => {
                return Err(ApiError::InvalidToken);
            }

            StatusCode::FORBIDDEN => {
                return Err(ApiError::Forbidden);
            }

            StatusCode::NOT_FOUND => {
                return Err(ApiError::NotFound);
            }

            // only one that is allowed to pass
            StatusCode::OK => (),

            v => {
                if let Ok(error) = error {
                    return Err(ApiError::ErrorMessage {
                        status: v,
                        error: error.error,
                        message: error.message,
                    });
                } else {
                    return Err(ApiError::StatusCode(v));
                }
            }
        }

        if let Ok(error) = error {
            return Err(ApiError::ErrorMessage {
                status: StatusCode::OK,
                error: error.error,
                message: error.message,
            });
        }

        let data = serde_json::from_str(&text)?;

        Ok(data)
    }
}
