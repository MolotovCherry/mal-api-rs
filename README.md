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

Please report bugs if you encounter any.

To get api client id and secret - which is required to use this - sign up for an app at https://myanimelist.net/apiconfig

For help understanding this api and how it works, please carefully read mal api docs at https://myanimelist.net/apiconfig/references/api/v2

Rate limiting is unspecified in mal api, but in practice you should do no more than 1 query/s

```rust
// if you want to create your token with a scope, add scope before generating a token
// mal should add this by default, so I don't think adding this manually is needed
client.auth.add_scope(Scope::new("write:users"));

// this requires a webserver to receive the oauth code+state for regenerate
// set your own custom callback for production usage
client.auth.set_callback(|url, state| async {
    // the url passed in is the one the client needs to navigate to

    // receive the state on your webserver, compare it to the received state above
    // to ensure it's valid and the right client. if you return wrong state, the
    // regenerate api will fail due to security check

    // get the code / state and return it
    (AuthorizationCode::new("".to_owned()), CsrfToken::new("".to_owned()))
});

// regenerate tokens from scratch
client.auth.regenerate().await;

// if you have a refresh key, you can exchange it for an access token
client.auth.refresh_token().await;

// you can automatically try to refresh it if access token expired
// if no refresh token exists, it will regenerate the whole thing
client.auth.try_refresh().await;

// you can also set the access/refresh token manually if you need to
client.auth.set_refresh_token(Some(RefreshToken::new("token")));
client.auth.set_access_token(Some(AccessToken::new("token")));
// set the time from Instant::now() after which access token expires
client.auth.set_expires_in(Some(Duration::from_secs(3600)));
client.auth.set_refresh_expires_in(Some(Duration::from_secs(3600)));

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
