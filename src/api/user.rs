use const_format::formatcp;
use itertools::Itertools as _;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{
    api_request::ApiError,
    objects::{User, Username},
    MalClient, API_URL,
};

const USER_URL: &str = formatcp!("{API_URL}/users/{{USER_NAME}}");

#[derive(Debug, Clone)]
pub struct UserApi<'a> {
    client: &'a MalClient,
}

impl<'a> UserApi<'a> {
    pub(crate) fn new(mal_client: &'a MalClient) -> Self {
        Self { client: mal_client }
    }

    /// User get endpoints
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/user>
    pub fn get(&self) -> UserApiGet<'a> {
        UserApiGet {
            client: self.client,
        }
    }
}

pub struct UserApiGet<'a> {
    client: &'a MalClient,
}

impl<'a> UserApiGet<'a> {
    /// GET user information
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#operation/users_user_id_get>
    pub fn information(self) -> UserInformationGet<'a> {
        UserInformationGet {
            client: self.client,
            fields: None,
        }
    }
}

/// GET user information
///
/// <https://myanimelist.net/apiconfig/references/api/v2#operation/users_user_id_get>
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct UserInformationGet<'a> {
    #[serde(skip)]
    client: &'a MalClient,

    fields: Option<String>,
}

impl<'a> UserInformationGet<'a> {
    pub fn fields<I: IntoIterator<Item = impl AsRef<str>>>(mut self, fields: I) -> Self {
        let fields = fields.into_iter().map(|f| f.as_ref().to_string()).join(",");

        self.fields = Some(fields);
        self
    }

    /// Send the request.
    pub async fn send(self) -> Result<User, ApiError> {
        let url = USER_URL.replace("{USER_NAME}", &Username::Me.to_string());
        let query = serde_qs::to_string(&self)?;
        let url = format!("{url}?{query}");

        self.client.api_request().get(url, true).await
    }

    /// Send the request.
    #[cfg(feature = "blocking")]
    pub fn send_blocking(self) -> Result<User, ApiError> {
        crate::RUNTIME.block_on(self.send())
    }
}
