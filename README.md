# MyAnimeList Rust Api

A fully typed Rust MyAnimeList api

Currently this has full coverage of the following apis:

- anime
- manga
- user_animelist
- user_mangalist
- user
- forum

The api is fully async, but has a blocking api for those who need sync (use the `*_blocking` function variants instead).

To get api client id and secret - which is required to use this - sign up for an app at https://myanimelist.net/apiconfig

For help understanding this api and how it works, please carefully read mal api docs at https://myanimelist.net/apiconfig/references/api/v2

Rate limiting is unspecified in mal api, but in practice you should do no more than 1 query/s

```rust
let secret = ClientSecret::new("secret".to_owned());
let id = ClientId::new("id".to_owned());
let redirect_url = RedirectUrl::new("url".to_owned());

let auth = Auth::new(id.clone(), secret, redirect_url);

// do your authentication for the client

// if you want to create your token with a scope, add scope before generating a token
// mal should add this by default, so I don't think adding this manually is needed
auth.add_scope(Scope::new("write:users"));

// regenerate tokens from scratch
{
    let auth_req = auth.authenticate();

    // verify auth_req.state() matches the state
    // you receive from the redirect url

    // state we received from client redirect url
    let client_state = CsrfToken::new("<state>".to_owned());
    // authorization code received from redirect url
    let auth_code = AuthorizationCode::new("<auth_code>".to_owned());
    // if client state does not match auth req, this will fail
    auth.authenticate_finish(client_state, auth_code).await;
}

// if you have a refresh key, you can exchange it for an access token
auth.refresh_token().await;

// you can automatically try to refresh it if access token expired
// if no refresh token exists, it will regenerate the whole thing
auth.try_refresh().await;

// you can also set the access/refresh token manually if you need to
auth.set_refresh_token_unchecked(RefreshToken::new("token"));
auth.set_access_token_unchecked(AccessToken::new("token"));
// set the time from Instant::now() after which access token expires
auth.set_expires_in_unchecked(Duration::from_secs(3600));
auth.set_refresh_expires_in_unchecked(Duration::from_secs(3600));

// ok great, I have done user auth and have access tokens
let client = MalClient::builder().auth_tokens(auth.to_tokens()).client_id(id).build().unwrap();

// maybe I want to change the tokens on the client later
// or I have a different user to do queries for
client.set_tokens(auth.to_tokens());

// use the api
client.anime().get().list().query("foo").send().await;

// for more information on the api, see their api docs:
// https://myanimelist.net/apiconfig/references/api/v2
//
// this api follows a builder pattern, and follows their api
// it should be relatively intuitive to use
//
// view mal docs to see if your token needs a scope or not,
// and for information on what the routes do
```

Warning: This crate may change api between versions before 1.0 as the api is fleshed out.

If you see any bugs, please report them. Mostly these may be instances "null" appears in the api but was assumed to always exist in the deserialized types. While this was quickly tested with a bit of data to hopefully get a complete picture, this is not guaranteed to be always correct. Other than that, it should be feature complete and working.
