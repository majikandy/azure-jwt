//! # A library that authenticates Azure JWT tokens.
//!
//! This library will fetch public keys from Microsoft and use those keys to validate the 
//! authenticity of a token you provide. It defaults to validating and mapping Azure Id tokens for
//! you out of the box, but should work with other tokens as well if you use a custom validator.
//!
//! # Dafault validation
//!
//! **There are mainly six conditions a well formed token will need to meet to be validated:**
//! 1. That the token is issued by Azure and is not tampered with
//! 2. That this token is issued for use in your application
//! 3. That the token is not expired
//! 4. That the token is not used before it's valid
//! 5. That the token is not issued in the future
//! 6. That the algorithm in the token header is the same as we use*
//!
//! * Note that we do NOT use the token header to set the algorithm for us, look [at this article
//! for more information on why that would be bad](https://auth0.com/blog/critical-vulnerabilities-in-json-web-token-libraries/)
//!
//! The validation will `Error` on a failed validation providing more granularity for library users
//! to find out why the token was rejected.
//!
//! If the token is invalid it will return an Error instead of a boolean. The main reason for this
//! is easier logging of what type of test it failed.
//! 
//! You also have a `validate_custom` mathod which gives you full control over the mapping of the token
//! fields and more control over the validation.
//!
//! # Security
//! You will need a private app_id created by Azure for your application to be able to veriify that
//! the token is created for your application (and not anyone with a valid Azure token can log in)
//! and you will need to authenticate that the user has the right access to your system.
//!
//! For more information, see this artice: https://docs.microsoft.com/en-us/azure/active-directory/develop/id-tokens
//! 
//! ## Features
//!- `vendored` feature will compile OpenSSL with the `vendored` feature: https://docs.rs/openssl/0.10.20/openssl/, but needs to
//!be used with the `default-features = false` flag or an error will occur.
//!
//!```toml
//!
//!azure_jwt = {version="0.1, default-features = false,  features = ["vendored"]}
//!
//!```
//! 
//! # Example
//! 
//! ```rust
//! use azure_jwt::*;
//! # let token = "ewogICAgICAgICAgICAgICAgInR5cCI6ICJKV1QiLAogICAgICAgICAgICAgICAgImFsZyI6ICJSUzI1NiIsCiAgICAgICAgICAgICAgICAia2lkIjogImk2bEdrM0ZaenhSY1ViMkMzbkVRN3N5SEpsWSIKICAgICAgICAgICAgfQ==.ewogICAgICAgICAgICAgICAgImF1ZCI6ICI2ZTc0MTcyYi1iZTU2LTQ4NDMtOWZmNC1lNjZhMzliYjEyZTMiLAogICAgICAgICAgICAgICAgImlzcyI6ICJodHRwczovL2xvZ2luLm1pY3Jvc29mdG9ubGluZS5jb20vNzJmOTg4YmYtODZmMS00MWFmLTkxYWItMmQ3Y2QwMTFkYjQ3L3YyLjAiLAogICAgICAgICAgICAgICAgImlhdCI6IDE1NTU4OTYyNDAsCiAgICAgICAgICAgICAgICAibmJmIjogMTU1NTg5NTI0MCwKICAgICAgICAgICAgICAgICJleHAiOiAxNTY0NTM3MjQwLAogICAgICAgICAgICAgICAgImFpbyI6ICJBWFFBaS84SUFBQUF0QWFaTG8zQ2hNaWY2S09udHRSQjdlQnE0L0RjY1F6amNKR3hQWXkvQzNqRGFOR3hYZDZ3TklJVkdSZ2hOUm53SjFsT2NBbk5aY2p2a295ckZ4Q3R0djMzMTQwUmlvT0ZKNGJDQ0dWdW9DYWcxdU9UVDIyMjIyZ0h3TFBZUS91Zjc5UVgrMEtJaWpkcm1wNjlSY3R6bVE9PSIsCiAgICAgICAgICAgICAgICAiYXpwIjogIjZlNzQxNzJiLWJlNTYtNDg0My05ZmY0LWU2NmEzOWJiMTJlMyIsCiAgICAgICAgICAgICAgICAibmFtZSI6ICJBYmUgTGluY29sbiIsCiAgICAgICAgICAgICAgICAiYXpwYWNyIjogIjAiLAogICAgICAgICAgICAgICAgIm9pZCI6ICI2OTAyMjJiZS1mZjFhLTRkNTYtYWJkMS03ZTRmN2QzOGU0NzQiLAogICAgICAgICAgICAgICAgInByZWZlcnJlZF91c2VybmFtZSI6ICJhYmVsaUBtaWNyb3NvZnQuY29tIiwKICAgICAgICAgICAgICAgICJyaCI6ICJJIiwKICAgICAgICAgICAgICAgICJzY3AiOiAiYWNjZXNzX2FzX3VzZXIiLAogICAgICAgICAgICAgICAgInN1YiI6ICJIS1pwZmFIeVdhZGVPb3VZbGl0anJJLUtmZlRtMjIyWDVyclYzeERxZktRIiwKICAgICAgICAgICAgICAgICJ0aWQiOiAiNzJmOTg4YmYtODZmMS00MWFmLTkxYWItMmQ3Y2QwMTFkYjQ3IiwKICAgICAgICAgICAgICAgICJ1dGkiOiAiZnFpQnFYTFBqMGVRYTgyUy1JWUZBQSIsCiAgICAgICAgICAgICAgICAidmVyIjogIjIuMCIKICAgICAgICAgICAgfQ==.0KR1BGZcrMAgs4pYgRcxFQGAuBPMv7Jwacms3XB2b3I8XvuKSdFR_lZCz9nVyy3Z5Ng5GC9kZl11Ufu4znGThBK4NVAuTYijGUK84bNDuSo6TOnA1flQb9ovZieAVOq5sI0dNHO8tqE_xesbsh5A8p4JKhTiTVAfTAVLc9_GcXwKuIJj-Mq9WTs5sfc3qZ678oCLRMwLKuYp8MoC7EzUKPIDj2qxa4Z6_SVveMmq2e3Bqlnk9pCRIrauJfoNTnC7qr57Coq4RCVUBxwAGH7FmM6Q_Q0s-rHhVL0z6o-WrmywmlBTnpmJrhBnd27a4rn_mUeHveBIhv-_OPxjAUVVKw";
//! # let n: &str = "AOx0GOQcSt5AZu02nlGWUuXXppxeV9Cu_9LcgpVBg_WQb-5DBHZpqs8AMek5u5iI4hkHCcOyMbQrBsDIVa9xxZxR2kq_8GtERsnd6NClQimspxT1WVgX5_WCAd5rk__Iv0GocP2c_1CcdT8is2OZHeWQySyQNSgyJYg6Up7kFtYabiCyU5q9tTIHQPXiwY53IGsNvSkqbk-OsdWPT3E4dqp3vNraMqXhuSZ-52kLCHqwPgAsbztfFJxSAEBcp-TS3uNuHeSJwNWjvDKTPy2oMacNpbsKb2gZgzubR6hTjvupRjaQ9SHhXyL9lmSZOpCzz2XJSVRopKUUtB-VGA0qVlk";
//! # let e: &str = "AQAB";
//! 
//! # let key = Jwk {
//! #         kid: "i6lGk3FZzxRcUb2C3nEQ7syHJlY".to_string(),
//! #         n: n.to_string(),
//! #         e: e.to_string(),
//! #     };
//! 
//!     let mut az_auth = AzureAuth::new("6e74172b-be56-4843-9ff4-e66a39bb12e3").unwrap();
//! #     az_auth.set_public_keys(vec![key]);
//! 
//!     let decoded_token = az_auth.validate_token(&token).expect("validated");
//!     assert_eq!(decoded_token.claims.preferred_username, Some("abeli@microsoft.com".to_string()));
//! ```
//! 
//! # Example in webserver
//! 
//! ```rust, ignore
//! struct AppState {
//!     azure_auth: auth::AzureAuth,
//! }
//! 
//! pub fn start_web_server(port: &str) -> Result<(), Error> {
//! 
//!     // since this calls windows api, wrap in Arc<Mutex<_>> and share the validator
//!     let app_state = Arc::new(Mutex::new(AppState {
//!         azure_auth: auth::AzureAuth::new("32166c25-5e31-4cfc-a29b-04d0dfdb019a").unwrap(),
//!     }));
//!     println!("Starting web server on: http://localhost:8000");
//! 
//!     server::new(move || app(app_state.clone())).bind(port)?.run();
//! 
//!     Ok(())
//! }
//! ```
//! # OpenSSL
//! 
//! This library depends on the [openssl crate](https://docs.rs/openssl/0.10.20/openssl/).
//! There are two options:
//! 1. If you have an installation of OpenSSL installed you can most likely compile this library with
//! its default settings.
//! 2. If you don't have OpenSSL libraries installed you can use the `vendored` feature that will in turn
//! compile the OpenSSL with its `vendored` feature enabled. This will compile and statically link 
//! OpenSSL to the library. You will need a C compiler, Make and Perl installed for it to build.
//! 
//! You'll find  more information here: https://docs.rs/openssl/0.10.20/openssl/
//! 
//! # Windows
//! 
//! On windows, the `vendored` feature requires a small workaround to find the systems root certificates
//! so we will add an additional dependency to fix that. For more information see: https://github.com/alexcrichton/openssl-probe
//! 
//! # Note
//! There is another library providing the same functionality but on a slightly lower level. If you 
//! reauire more control then have a look at: https://github.com/tazjin/alcoholic_jwt
use base64;
use chrono::{Duration, Local, NaiveDateTime};
use jsonwebtoken as jwt;
use reqwest::{self, Response};
use serde::{Deserialize, Serialize};
#[cfg(target_os = "windows")] 
use openssl_probe;
#[cfg(feature = "vendored")] 
use openssl_vendored as openssl;
#[cfg(not(feature = "vendored"))] 
use openssl_std as openssl;
use openssl::rsa::Rsa;


mod error;
pub use error::AuthErr;

const AZ_OPENID_URL: &str =
    "https://login.microsoftonline.com/common/.well-known/openid-configuration";

#[cfg(target_os="windows")] 
fn init() {
    openssl_probe::init_ssl_cert_env_vars();
}

#[cfg(not(windows))]
fn init() {
}

/// AzureAuth is the what you'll use to validate your token. I'll briefly explain here what
/// defaults are set and which you can change:
/// 
///
/// # Defaults
///
/// - Public key expiration: dafault set to 24h, use `set_expiration` to set a different expiration
///   in hours.
/// - Hashing algorithm: Sha256, you can't change this setting. Submit an issue in the github repo
///   if this is important to you
/// - Retry on no match. If no matching key is found and our keys are older than an hour, we
///   refresh the keys and try once more. Limited to once in an hour. You can disable this by
///   calling `set_no_retry()`.
/// - The timestamps are given a 60s "leeway" to account for time skew between servers
///
/// # Errors:
/// - If one of Microsofts enpoints for public keys are down
/// - If the token can't be parsed as a valid Azure token
/// - If the tokens fails it's authenticity test
/// - If the token is invalid
#[derive(Debug, Clone)]
pub struct AzureAuth {
    aud_to_val: String,
    jwks_uri: String,
    public_keys: Option<Vec<Jwk>>,
    last_refresh: Option<NaiveDateTime>,
    exp_hours: i64,
    retry_counter: u32,
    is_retry_enabled: bool,
    is_offline: bool,
}

impl AzureAuth {
    /// One thing to note that this method will call the Microsoft apis to fetch the current keys
    /// an this can fail. The public keys are fetched since we will not be able to perform any
    /// verification without them. Please note that this method is quite expensive to do. Try
    /// keeping the object alive instead of creating new objects. If you need to pass around an
    /// instance of the object, then cloning it will be cheaper than creating a new one.
    ///
    /// # Errors
    /// If there is a connection issue to the Microsoft public key apis.
    pub fn new(aud: impl Into<String>) -> Result<Self, AuthErr> {
        init();

        Ok(AzureAuth {
            aud_to_val: aud.into(),
            jwks_uri: AzureAuth::get_jwks_uri()?,
            public_keys: None,
            last_refresh: None,
            exp_hours: 24,
            retry_counter: 0,
            is_retry_enabled: true,
            is_offline: false,
        })
    }

    /// Does not call the Microsoft openid configuration endpoint or fetches the JWK set.
    /// Use this if you want to handle updating the public keys yourself
    fn new_offline(aud: impl Into<String>, public_keys: Vec<Jwk>) -> Result<Self, AuthErr> {
        Ok(AzureAuth {
            aud_to_val: aud.into(),
            jwks_uri: String::new(),
            public_keys: Some(public_keys),
            last_refresh: Some(Local::now().naive_local()),
            exp_hours: 24,
            retry_counter: 0,
            is_retry_enabled: true,
            is_offline: true,
        })
    }

    /// Dafault validation, see struct documentation for the defaults.
    pub fn validate_token(&mut self, token: &str) -> Result<Token<AzureJwtClaims>, AuthErr> {
        let mut validator = jwt::Validation::new(jwt::Algorithm::RS256);

        // exp, nbf, iat is set to validate as default
        validator.leeway = 60;
        validator.set_audience(&self.aud_to_val);
        let decoded: Token<AzureJwtClaims> = self.validate_token_authenticity(token, &validator)?;

        Ok(decoded)
    }

    /// Allows for a custom validator and mapping the token to your own type.
    /// Useful in situations where you get fields you that are not covered by
    /// the default mapping or want to change the validaion requirements (i.e
    /// if you want the leeway set to two minutes instead of one).
    /// 
    /// # Note
    /// You'll need to pull in `jsonwebtoken` to use `Validation` from that crate.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use azure_oauth_r1s::*;
    /// use jsonwebtoken::{Validation, Token};
    /// use serde::{Seralize, Deserialize};
    ///
    /// let mut validator = Validation::new();
    /// validator.leeway = 120;
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct MyClaims {
    ///     group: String,
    ///     roles: Vec<String>,
    /// }
    ///
    /// let auth = AzureAuth::new(my_client_id_from_azure).unwrap();
    ///
    /// let valid_token: Token<MyClaims>  = auth.validate_custom(some_token, &validator).unwrap();
    /// ```
    ///
    pub fn validate_custom<T>(
        &mut self,
        token: &str,
        validator: &jwt::Validation,
    ) -> Result<Token<T>, AuthErr>
    where
        for<'de> T: Serialize + Deserialize<'de>,
    {
        let decoded: Token<T> = self.validate_token_authenticity(token, &validator)?;
        Ok(decoded)
    }

    fn validate_token_authenticity<T>(
        &mut self,
        token: &str,
        validator: &jwt::Validation,
    ) -> Result<Token<T>, AuthErr>
    where
        for<'de> T: Serialize + Deserialize<'de>,
    {
        // if we´re in offline, we never refresh the keys. It's up to the user to do that.
        if !self.is_keys_valid() && !self.is_offline {
            self.refresh_pub_keys()?;
        }
        // does not validate the token!
        let decoded = jwt::decode_header(token)?;

        let key = match &self.public_keys {
            None => return Err(AuthErr::Other("Internal err. No public keys found.".into())),
            Some(keys) => match &decoded.kid {
                None => return Err(AuthErr::Other("No `kid` in token.".into())),
                Some(kid) => keys.iter().find(|k| k.kid == *kid),
            },
        };

        let auth_key = match key {
            None => {
                // the first time this happens let's go and refresh the keys and try once more.
                // It could be that our keys are out of date. Limit to once in an hour.
                if self.should_retry() {
                    self.refresh_pub_keys()?;
                    self.retry_counter += 1;
                    self.validate_token(token)?;
                    unreachable!()
                } else {
                    self.retry_counter = 0;
                    return Err(AuthErr::Other(
                        "Invalid token. Could not verify authenticity.".into(),
                    ));
                }
            }
            Some(key) => {
                self.retry_counter = 0;
                key
            }
        };

        // the jwt library expects a byte input so we need to decode the
        // base64 data to an bytearray
        let auth_key_bytes = auth_key.get_public_key()?;
        //let key_as_bytes = from_base64_to_bytearray(&auth_key)?;

        let valid: Token<T> = jwt::decode(token, &auth_key_bytes, &validator)?;

        Ok(valid)
    }

    fn should_retry(&mut self) -> bool {
        if self.is_offline || !self.is_retry_enabled {
            return false;
        }

        match &self.last_refresh {
            Some(lr) => {
                self.retry_counter == 0 && Local::now().naive_local() - *lr > Duration::hours(1)
            }
            None => false,
        }
    }

    /// Sets the expiration of the cached public keys in hours. Pr. 04.2019 Microsoft rotates these
    /// every 24h.
    pub fn set_expiration(&mut self, hours: i64) {
        self.exp_hours = hours;
    }

    pub fn set_no_retry(&mut self) {
        self.is_retry_enabled = false;
    }

    fn is_keys_valid(&self) -> bool {
        match self.last_refresh {
            None => false,
            Some(lr) => (Local::now().naive_local() - lr) <= Duration::hours(self.exp_hours),
        }
    }

    fn refresh_pub_keys(&mut self) -> Result<(), AuthErr> {
        let mut resp: Response = reqwest::get(&self.jwks_uri)?;
        let resp: JwkSet = resp.json()?;
        self.last_refresh = Some(Local::now().naive_local());
        self.public_keys = Some(resp.keys);
        Ok(())
    }

    fn refresh_rwks_uri(&mut self) -> Result<(), AuthErr> {
        self.jwks_uri = AzureAuth::get_jwks_uri()?;
        Ok(())
    }

    fn get_jwks_uri() -> Result<String, AuthErr> {
        let mut resp: Response = reqwest::get(AZ_OPENID_URL)?;
        let resp: OpenIdResponse = resp.json()?;

        Ok(resp.jwks_uri)
    }

    /// If you use the "offline" variant you'll need this to update the public keys, if you don't
    /// use the offline version you probably don't want to change these unless you're testing.
    pub fn set_public_keys(&mut self, pub_keys: Vec<Jwk>) {
        self.last_refresh = Some(Local::now().naive_local());
        self.public_keys = Some(pub_keys);
    }
}

pub struct AzureJwtHeader {
    /// Indicates that the token is a JWT.
    pub typ: String,
    /// Indicates the algorithm that was used to sign the token. Example: "RS256"
    pub alg: String,
    /// Thumbprint for the public key used to sign this token. Emitted in both
    /// v1.0 and v2.0 id_tokens
    pub kid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureJwtClaims {
    /// dentifies the intended recipient of the token. In id_tokens, the audience
    /// is your app's Application ID, assigned to your app in the Azure portal.
    /// Your app should validate this value, and reject the token if the value
    /// does not match.
    pub aud: String,

    /// The application ID of the client using the token. The application can
    /// act as itself or on behalf of a user. The application ID typically
    /// represents an application object, but it can also represent a service
    /// principal object in Azure AD.
    pub azp: Option<String>,

    /// Indicates how the client was authenticated. For a public client, the
    /// value is "0". If client ID and client secret are used, the value is "1".
    /// If a client certificate was used for authentication, the value is "2".
    pub azpacr: Option<String>,

    /// Identifies the security token service (STS) that constructs and returns
    /// the token, and the Azure AD tenant in which the user was authenticated.
    /// If the token was issued by the v2.0 endpoint, the URI will end in /v2.0.
    /// The GUID that indicates that the user is a consumer user from a Microsoft
    /// account is 9188040d-6c67-4c5b-b112-36a304b66dad.
    ///
    /// Your app should use the GUID portion of the claim to restrict the set of
    /// tenants that can sign in to the app, if applicable.
    pub iss: String,

    /// Unix timestamp. "Issued At" indicates when the authentication for this
    /// token occurred.
    pub iat: u64,

    /// Records the identity provider that authenticated the subject of the token.
    /// This value is identical to the value of the Issuer claim unless the user
    /// account not in the same tenant as the issuer - guests, for instance. If
    /// the claim isn't present, it means that the value of iss can be used
    /// instead. For personal accounts being used in an organizational context
    /// (for instance, a personal account invited to an Azure AD tenant), the idp
    /// claim may be 'live.com' or an STS URI containing the Microsoft account
    /// tenant 9188040d-6c67-4c5b-b112-36a304b66dad
    pub idp: Option<String>,

    /// Unix timestamp. The "nbf" (not before) claim identifies the time before
    /// which the JWT MUST NOT be accepted for processing.
    pub nbf: u64,

    /// Unix timestamp. he "exp" (expiration time) claim identifies the
    /// expiration time on or after which the JWT MUST NOT be accepted for
    /// processing. It's important to note that a resource may reject the token
    /// before this time as well - if, for example, a change in authentication
    /// is required or a token revocation has been detected.
    pub exp: u64,

    /// The code hash is included in ID tokens only when the ID token is issued
    /// with an OAuth 2.0 authorization code. It can be used to validate the
    /// authenticity of an authorization code. For details about performing this
    /// validation, see the OpenID Connect specification.
    pub c_hash: Option<String>,

    /// The access token hash is included in ID tokens only when the ID token is
    /// issued with an OAuth 2.0 access token. It can be used to validate the
    /// authenticity of an access token. For details about performing this
    /// validation, see the OpenID Connect specification.
    pub at_hash: Option<String>,

    /// The email claim is present by default for guest accounts that have an
    /// email address. Your app can request the email claim for managed users
    /// (those from the same tenant as the resource) using the email optional
    /// claim. On the v2.0 endpoint, your app can also request the email OpenID
    /// Connect scope - you don't need to request both the optional claim and
    /// the scope to get the claim. The email claim only supports addressable
    /// mail from the user's profile information.
    pub preferred_username: Option<String>,

    /// The name claim provides a human-readable value that identifies the
    /// subject of the token. The value isn't guaranteed to be unique, it is
    /// mutable, and it's designed to be used only for display purposes. The
    /// profile scope is required to receive this claim.
    pub name: Option<String>,

    /// The nonce matches the parameter included in the original /authorize
    /// request to the IDP. If it does not match, your application should reject
    /// the token.
    pub nonce: Option<String>,

    /// Guid. The immutable identifier for an object in the Microsoft identity system,
    /// in this case, a user account. This ID uniquely identifies the user
    /// across applications - two different applications signing in the same
    /// user will receive the same value in the oid claim. The Microsoft Graph
    /// will return this ID as the id property for a given user account. Because
    /// the oid allows multiple apps to correlate users, the profile scope is
    /// required to receive this claim. Note that if a single user exists in
    /// multiple tenants, the user will contain a different object ID in each
    /// tenant - they're considered different accounts, even though the user
    /// logs into each account with the same credentials.
    pub oid: String,

    /// The set of roles that were assigned to the user who is logging in.
    pub roles: Option<Vec<String>>,

    /// The set of scopes exposed by your application for which the client
    /// application has requested (and received) consent. Your app should verify
    /// that these scopes are valid ones exposed by your app, and make authorization
    /// decisions based on the value of these scopes. Only included for user tokens.
    pub scp: Option<String>,

    /// The principal about which the token asserts information, such as the
    /// user of an app. This value is immutable and cannot be reassigned or
    /// reused. The subject is a pairwise identifier - it is unique to a
    /// particular application ID. If a single user signs into two different
    /// apps using two different client IDs, those apps will receive two
    /// different values for the subject claim. This may or may not be wanted
    /// depending on your architecture and privacy requirements.
    pub sub: String,

    /// A GUID that represents the Azure AD tenant that the user is from.
    /// For work and school accounts, the GUID is the immutable tenant ID of
    /// the organization that the user belongs to. For personal accounts,
    /// the value is 9188040d-6c67-4c5b-b112-36a304b66dad. The profile scope is
    /// required to receive this claim.
    pub tid: String,

    /// Provides a human readable value that identifies the subject of the
    /// token. This value isn't guaranteed to be unique within a tenant and
    /// should be used only for display purposes. Only issued in v1.0 id_tokens.
    pub unique_name: Option<String>,

    /// Indicates the version of the id_token. Either 1.0 or 2.0.
    pub ver: String,
}

/// URL-safe character set without padding that allows trailing bits,
/// which appear in some JWT implementations.
fn from_base64_to_bytearray(b64_str: &str) -> Result<Vec<u8>, AuthErr> {
    let b64_config = base64::URL_SAFE_NO_PAD.decode_allow_trailing_bits(true);

    let decoded = base64::decode_config(b64_str, b64_config)
        .map_err(|e| AuthErr::ParseError(e.to_string()))?;
    Ok(decoded)
}

fn from_base64_to_bytearray_non_url(b64_str: &str) -> Result<Vec<u8>, AuthErr> {
    let decoded = base64::decode_config(b64_str, base64::STANDARD)
        .map_err(|e| AuthErr::ParseError(e.to_string()))?;
    Ok(decoded)
}

#[derive(Debug, Deserialize)]
struct JwkSet {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jwk {
    pub kid: String,
    pub n: String,
    pub e: String,
}

impl Jwk {
    pub fn get_public_key(&self) -> Result<Vec<u8>, AuthErr> {
        let n = from_base64_to_bytearray(&self.n)?;
        let e = from_base64_to_bytearray(&self.e)?;
        let n = openssl::bn::BigNum::from_slice(&n)?;
        let e = openssl::bn::BigNum::from_slice(&e)?;

        let key = Rsa::from_public_components(n, e)?;
        let key_bytes = key.public_key_to_der_pkcs1()?;
        Ok(key_bytes)
    }
}

#[derive(Deserialize)]
struct OpenIdResponse {
    jwks_uri: String,
}

type Token<T> = jwt::TokenData<T>;

#[cfg(test)]
mod tests {
    use super::*;

    const PUBLIC_KEY_N: &str = "AOx0GOQcSt5AZu02nlGWUuXXppxeV9Cu_9LcgpVBg_WQb-5DBHZpqs8AMek5u5iI4hkHCcOyMbQrBsDIVa9xxZxR2kq_8GtERsnd6NClQimspxT1WVgX5_WCAd5rk__Iv0GocP2c_1CcdT8is2OZHeWQySyQNSgyJYg6Up7kFtYabiCyU5q9tTIHQPXiwY53IGsNvSkqbk-OsdWPT3E4dqp3vNraMqXhuSZ-52kLCHqwPgAsbztfFJxSAEBcp-TS3uNuHeSJwNWjvDKTPy2oMacNpbsKb2gZgzubR6hTjvupRjaQ9SHhXyL9lmSZOpCzz2XJSVRopKUUtB-VGA0qVlk";
    const PUBLIC_KEY_E: &str = "AQAB";

    const PRIVATE_KEY_TEST: &str =
"MIIEowIBAAKCAQEA7HQY5BxK3kBm7TaeUZZS5demnF5X0K7/0tyClUGD9ZBv7kME\
dmmqzwAx6Tm7mIjiGQcJw7IxtCsGwMhVr3HFnFHaSr/wa0RGyd3o0KVCKaynFPVZ\
WBfn9YIB3muT/8i/Qahw/Zz/UJx1PyKzY5kd5ZDJLJA1KDIliDpSnuQW1hpuILJT\
mr21MgdA9eLBjncgaw29KSpuT46x1Y9PcTh2qne82toypeG5Jn7naQsIerA+ACxv\
O18UnFIAQFyn5NLe424d5InA1aO8MpM/Lagxpw2luwpvaBmDO5tHqFOO+6lGNpD1\
IeFfIv2WZJk6kLPPZclJVGikpRS0H5UYDSpWWQIDAQABAoIBAQC982Yrmi7q7IHC\
/qWglUpzKhLGe2PAWVVaZ5rfnIoNs8K3fU8QcUKumFGAMsjpeM1pnaXSeExFmGsM\
Y+Ox1YwSUA81DYxuH6Ned86YDqpgIDr5M0Ba7JmDOLWXoIR8byB19oMOuhjBAW+P\
EKlb0Z2a1f1Gt3J8oAxWq8PDsShHRdjyesVS36QZpIgjZskcNws/zqqqDRrLWuLm\
Avk6E+tMD6sqo9xpzEqHF7rmwtt5yAtM1oZdWoEg2O+wZH5DBX2GhLlNZi/8sIiF\
Mo+jouQn+l6Qc4G65vnnoZ+yEuf9fTJPnTHBFMViUcmTPsdbD4eLfrRXwAE9GYrv\
R/RVusABAoGBAPgsQ4kAChpzU2aP21NQV1XTBW+eoHVbcJoYuOlmwB6x5o8lDUz/\
EQVVYZavfNY1AjhEkfltCDjm1GHyWofrtGKTy7DHSZwPw5CxuqDtaiC6PMpFEu+O\
xa09s7IZxpgInlrhY5JskOkH495BQ0xIU8UDxuP6sdtVNeQmWGjKG7kBAoGBAPPp\
Nid4QEV4XleyAXT/JQGugdpa7TirWOEATNo10YPPqz7GphRhucT0ipNKMi/0XKh3\
U0IC7XxjUvtE2LP9TVGAcV/Wzi4EYp1fziFuF9QcUds2tJ60SpfgIQrmVcF1zHxn\
4/mSABoIyFxZSb4Tq9f+KXPAO5/l0NjgrVwk6gVZAoGAbMVZxE4UH4u0XhtnEZkA\
7kjS9R0dTtKJA8EaKpIyWkG2v76JmdmhaCkH4LeBi5EoK+lB4YR8OhRRuawzKaeR\
JDOK7ywpgxEVsfFzzty/yyBVTIIBzqVQ1qFYhRLvC+ubHFH1BlQ3HyuqH9uS13hL\
3unM3lceZPdv61MzJJqQlAECgYAWg0MFV5sPDnIexAZQZzBiPFot7lCQ93fHpMBz\
L557/RIARFOV9AMyg6O6vpFtTa+zuPfNUvnajkxddthNnKajTCiqwOfc5Xi4r9wV\
x9SZNlfz1NPNBjUQWZaTK/lkVtwd63TmVyx9OqxLoc4lpikpUYM/9NFMC+k/61T0\
+U9EWQKBgCdZV3yxwkz3pi6/E40EXfUsj8HQG/UtFJGeUNQiysBrxTmtmwLyvJeC\
GruG96j1JcehpbcWKV+ObyMQuk65dM94uM7Wa+2NCA/MvorVcU7wdPbq7/eczZU4\
xMd+OWT6JsInVM1ASh1mcn+Q0/Z3WqxxetCQLqaMs+FATn059dGf";

    fn test_token_header() -> String {
        format!(
            r#"{{
                "typ": "JWT",
                "alg": "RS256",
                "kid": "i6lGk3FZzxRcUb2C3nEQ7syHJlY"
            }}"#
        )
    }

    fn test_token_claims() -> String {
        format!(
            r#"{{
                "aud": "6e74172b-be56-4843-9ff4-e66a39bb12e3",
                "iss": "https://login.microsoftonline.com/72f988bf-86f1-41af-91ab-2d7cd011db47/v2.0",
                "iat": {},
                "nbf": {},
                "exp": {},
                "aio": "AXQAi/8IAAAAtAaZLo3ChMif6KOnttRB7eBq4/DccQzjcJGxPYy/C3jDaNGxXd6wNIIVGRghNRnwJ1lOcAnNZcjvkoyrFxCttv33140RioOFJ4bCCGVuoCag1uOTT22222gHwLPYQ/uf79QX+0KIijdrmp69RctzmQ==",
                "azp": "6e74172b-be56-4843-9ff4-e66a39bb12e3",
                "name": "Abe Lincoln",
                "azpacr": "0",
                "oid": "690222be-ff1a-4d56-abd1-7e4f7d38e474",
                "preferred_username": "abeli@microsoft.com",
                "rh": "I",
                "scp": "access_as_user",
                "sub": "HKZpfaHyWadeOouYlitjrI-KffTm222X5rrV3xDqfKQ",
                "tid": "72f988bf-86f1-41af-91ab-2d7cd011db47",
                "uti": "fqiBqXLPj0eQa82S-IYFAA",
                "ver": "2.0"
            }}"#, 
        chrono::Utc::now().timestamp() - 1000,
        chrono::Utc::now().timestamp() - 2000,
        chrono::Utc::now().timestamp() + 1000)
    }

    // We create a test token from parts here. We use the v2 token used as example
    // in https://docs.microsoft.com/en-us/azure/active-directory/develop/id-tokens
    fn generate_test_token() -> String {
        // jwt library expects a `*.der` key wich is a byte encoded file so
        // we need to convert the key from base64 to their byte value to use them.
        let private_key = from_base64_to_bytearray_non_url(PRIVATE_KEY_TEST).expect("priv_key");

        // we need to construct the calims in a function since we need to set
        // the expiration relative to current time
        let test_token_playload = test_token_claims();
        let test_token_header = test_token_header();

        // we base64 (url-safe-base64) the header and claims and arrange
        // as a jwt payload -> header_as_base64.claims_as_base64
        let test_token = [
            base64::encode_config(&test_token_header, base64::URL_SAFE),
            base64::encode_config(&test_token_playload, base64::URL_SAFE),
        ]
        .join(".");

        // we create the signature using our private key
        let signature = jwt::sign(&test_token, &private_key, jwt::Algorithm::RS256).expect("Signed");

        let public_key = Jwk {
            kid: "".to_string(),
            n: PUBLIC_KEY_N.to_string(),
            e: PUBLIC_KEY_E.to_string(),
        };

        let public_key = public_key.get_public_key().expect("Get public key.");

        // we construct a complete token which looks like: header.claims.signature
        let complete_token = format!("{}.{}", test_token, signature);

        // we verify the signature here as well to catch errors in our testing
        // code early
        let verified = jwt::verify(&signature, &test_token, &public_key, jwt::Algorithm::RS256)
            .expect("verified");
        assert!(verified);

        complete_token
    }

    #[test]
    fn decode_token() {
        let token = generate_test_token();

        // we need to construct our own key object that matches on `kid` field
        // just as it should if we used the fetched keys from microsofts servers
        // since our validation methods converts the base64 data to bytes for us
        // we don't need to worry about that here.
        // let from_std = base64::decode_config(PUBLIC_KEY_TEST, base64::STANDARD).unwrap();
        // let to_url_safe = base64::encode_config(&from_std, base64::URL_SAFE);
        let key = Jwk {
            kid: "i6lGk3FZzxRcUb2C3nEQ7syHJlY".to_string(),
            n: PUBLIC_KEY_N.to_string(),
            e: PUBLIC_KEY_E.to_string(),
        };

        let mut az_auth =
            AzureAuth::new_offline("6e74172b-be56-4843-9ff4-e66a39bb12e3", vec![key]).unwrap();

        az_auth.validate_token(&token).unwrap();
    }

    // TODO: we need a test for the retry operation.

    #[test]
    fn refresh_rwks_uri() {
        let _az_auth = AzureAuth::new("app_secret").unwrap();
    }

    #[test]
    fn azure_ad_get_public_keys() {
        let mut az_auth = AzureAuth::new("app_secret").unwrap();
        az_auth.refresh_pub_keys().unwrap();
    }

    #[test]
    fn is_not_valid_more_than_24h() {
        let mut az_auth = AzureAuth::new("app_secret").unwrap();
        az_auth.last_refresh = Some(Local::now().naive_local() - Duration::hours(25));

        assert!(!az_auth.is_keys_valid());
    }

}
