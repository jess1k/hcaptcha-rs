//! Provides a struct to collect the required and optional parameters for
//! the hcaptcha api request.
//!
//! # Example
//!
//! ```
//!     use hcaptcha::HcaptchaRequest;
//! # #[tokio::main]
//! # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
//!     let secret = get_your_secret();         // your secret key
//!     let bad_token = get_user_token();       // user's response token
//!     let site_key = get_your_site_key();     // your site key
//!     let user_ip = get_user_ip_address();    // user's ip address
//!
//!     let request = HcaptchaRequest::new(&secret, &bad_token)? // <- returns error
//!         .set_site_key(&site_key)?
//!         .set_user_ip(&user_ip)?;
//! # Ok(())
//! # }
//! # fn get_your_secret() -> String {
//! #   "0x123456789abcde0f123456789abcdef012345678".to_string()
//! # }
//! # fn get_user_token() -> String {
//! #    "thisisnotapropertoken".to_string()
//! # }
//! # fn get_user_ip_address() -> String {
//! #    "192.168.71.17".to_string()
//! # }
//! # use uuid::Uuid;
//! # fn get_your_site_key() -> String {
//! #    Uuid::new_v4().to_string()
//! # }
//!
//! ```

use crate::domain::HcaptchaSecret;
use crate::HcaptchaCaptcha;
use crate::HcaptchaError;

/// Type to capture the required and optional data for a call to the hcaptcha API
#[allow(missing_doc_code_examples)]
#[derive(Debug, Default, serde::Serialize)]
pub struct HcaptchaRequest {
    /// The Hcaptcha response data returned from the client's call to
    /// Hcaptcha client side API. May include user_ip and site_key from
    /// the client.
    captcha: HcaptchaCaptcha,
    /// The secret_key related to the site_key used to capture the response.
    secret: HcaptchaSecret,
}

#[allow(missing_doc_code_examples)]
impl HcaptchaRequest {
    /// Create a new HcaptchaRequest
    ///
    /// # Input
    ///
    /// The Hcaptcha API has two mandatory paramaters:
    ///     secret:     The client's secret key for authentication
    ///     captcha:    [HcaptchaCaptch] struct including response
    ///
    /// # Output
    ///
    /// A HcaptchaRequest struct is returned if the input strings are valid.
    /// A HcaptchaError is returned if the validation fails.
    ///
    /// # Example
    ///
    /// ``` no_run
    ///     use hcaptcha::HcaptchaRequest;
    /// # fn main() -> Result<(), hcaptcha::HcaptchaError>{
    ///     let secret = get_your_secret();     // your secret key
    ///     let captcha = get_captcha();        // captcha struct with user's token
    ///
    ///     let request = HcaptchaRequest::new(&secret, captcha)?;
    /// # Ok(())
    /// # }
    /// # fn get_your_secret() -> String {
    /// #   "0x123456789abcde0f123456789abcdef012345678".to_string()
    /// # }
    /// # fn get_user_token() -> HcaptchaCaptcha {
    /// #    HcaptchaCaptcha::new("thisisnotapropertoken")
    /// # }
    ///  ```
    /// # Logging
    ///
    /// If the tracing feature is enabled a debug level span is set for the
    /// method.
    /// The secret field will not be logged.
    ///
    #[allow(dead_code)]
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(
            name = "Create new HcaptchaRequest from HcaptchaCaptcha struct.",
            skip(secret),
            level = "debug"
        )
    )]
    pub fn new(secret: &str, captcha: HcaptchaCaptcha) -> Result<HcaptchaRequest, HcaptchaError> {
        Ok(HcaptchaRequest {
            captcha,
            secret: HcaptchaSecret::parse(secret.to_owned())?,
        })
    }

    /// Create a new HcaptchaRequest from only the response string
    ///
    /// # Input
    ///
    /// The Hcaptcha API has two mandatory paramaters:
    ///     secret:     The client's secret key for authentication
    ///     response:    The response code to validate
    ///
    /// # Output
    ///
    /// A HcaptchaRequest struct is returned if the inputs are valid.
    /// A HcaptchaError is returned if the validation fails.
    ///
    /// # Example
    ///
    /// ``` no_run
    ///     use hcaptcha::HcaptchaRequest;
    /// # fn main() -> Result<(), hcaptcha::HcaptchaError>{
    ///     let secret = get_your_secret();     // your secret key
    ///     let response = get_user_token();    // Hcaptcha client response
    ///
    ///     let request = HcaptchaRequest::new(&secret, &response)?;
    /// # Ok(())
    /// # }
    /// # fn get_your_secret() -> String {
    /// #   "0x123456789abcde0f123456789abcdef012345678".to_string()
    /// # }
    /// # fn get_user_token() -> String {
    /// #    "response_string".to_owend()
    /// # }
    ///  ```
    /// # Logging
    ///
    /// If the tracing feature is enabled a debug level span is set for the
    /// method.
    /// The secret field will not be logged.
    ///
    #[allow(dead_code)]
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(
            name = "Create new HcaptchaRequest from response string.",
            skip(secret),
            level = "debug"
        )
    )]
    pub fn new_from_response(
        secret: &str,
        response: &str,
    ) -> Result<HcaptchaRequest, HcaptchaError> {
        let captcha = HcaptchaCaptcha::new(response)?;
        Ok(HcaptchaRequest::new(secret, captcha)?)
    }

    /// Specify the optional ip address value
    ///
    /// Update client IP adress.
    ///
    /// # Example
    /// ``` no_run
    ///     use hcaptcha::HcaptchaRequest;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
    ///     let secret = get_your_secret();         // your secret key
    ///     let response  = get_response();       // user's response token
    ///     let user_ip = get_user_ip_address();    // user's ip address
    ///
    ///     let request = HcaptchaRequest::new_from_response(&secret, &response)?
    ///         .set_user_ip(&user_ip)?;
    /// # Ok(())
    /// # }
    /// # fn get_your_secret() -> String {
    /// #   "0x123456789abcde0f123456789abcdef012345678".to_string()
    /// # }
    /// # fn get_user_token() -> String {
    /// #    "thisisnotapropertoken".to_string()
    /// # }
    /// # use std::net::{IpAddr, Ipv4Addr};
    /// # fn get_user_ip_address() -> String {
    /// #    "192.168.71.17".to_string()
    /// # }
    ///
    /// ```
    ///
    /// #Logging
    ///
    /// If the `trace` feature is enabled a debug level span is set for the
    /// method.
    /// The secret field is not logged.
    ///
    #[allow(dead_code)]
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(
            name = "Request verification from hcaptcha.",
            skip(self),
            fields(captcha = ?self.captcha),
            level = "debug"
        )
    )]
    pub fn set_user_ip(self, user_ip: &str) -> Result<Self, HcaptchaError> {
        let get_string = |x| -> &str {
            if let Some(y) = x {
                y.as_str()
            } else {
                ""
            }
        };

        let site_key = get_string(&self.captcha.site_key());
        let user_ip = get_string(&self.captcha.user_ip());

        let captcha = HcaptchaCaptcha::new(&self.captcha.response().as_str()).set_site_key;
        let request = HcaptchaRequest::new(&self.secret().as_str(), captcha);

        &self.captcha.set_user_ip(user_ip)?;
        Ok(request)
    }

    /// Specify the optional site_key value
    ///
    /// Update the site_key.
    ///
    /// # Example
    /// Create a new request and set the site_key field in the request.
    /// ```
    ///     use hcaptcha::HcaptchaRequest;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
    ///     let secret = get_your_secret();     // your secret key
    ///     let captcha = get_captcha();        // captcha
    ///     let site_key = get_your_site_key(); // your site key
    ///
    ///     let request = HcaptchaRequest::new(&secret, captcha)?
    ///         .set_site_key(&site_key);
    /// # Ok(())
    /// # }
    /// # fn get_your_secret() -> String {
    /// #   "0x123456789abcde0f123456789abcdef012345678".to_string()
    /// # }
    /// # fn get_captcha() -> HcaptchaCaptcha {
    /// #    HcaptchaCaptcha::new("thisisnotapropertoken")
    /// # }
    /// # use uuid::Uuid;
    /// # fn get_your_site_key() -> String {
    /// #    Uuid::new_v4().to_string()
    /// # }
    ///
    /// ```
    ///
    /// #Logging
    ///
    /// If the `trace` feature is enabled a debug level span is created for the
    /// method.
    /// The secret field is not logged.
    ///
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(
            name = "Request verification from hcaptcha.",
            skip(self),
            fields(captcha = ?self.captcha),
            level = "debug"
        )
    )]
    pub fn set_site_key(mut self, site_key: &str) -> Result<Self, HcaptchaError> {
        self.captcha.set_site_key(site_key)?;
        Ok(self)
    }

    pub(crate) fn secret(&self) -> HcaptchaSecret {
        self.secret
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    // use crate::error::Code::*;
    // use crate::HcaptchaError::*;
    // use std::collections::HashSet;
    use crate::HcaptchaCaptcha;
    use claim::{assert_none, assert_ok};
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::iter;

    fn random_hex_string(len: usize) -> String {
        let mut rng = thread_rng();
        let chars: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(len / 2)
            .collect();

        hex::encode(chars)
    }

    fn random_response() -> String {
        let mut rng = thread_rng();
        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(100)
            .collect()
    }

    fn dummy_captcha() -> HcaptchaCaptcha {
        HcaptchaCaptcha::new(&random_response())
            .unwrap()
            .set_user_ip(&fakeit::internet::ipv4_address())
            .unwrap()
            .set_site_key(&fakeit::unique::uuid_v4())
            .unwrap()
    }

    #[test]
    fn valid_new_from_captcha_struct() {
        let secret = format!("0x{}", random_hex_string(40));
        let captcha = dummy_captcha();

        assert_ok!(HcaptchaRequest::new(&secret, captcha));
    }

    #[test]
    fn valid_new_from_response() {
        let secret = format!("0x{}", random_hex_string(40));
        let response = random_response();

        let request = HcaptchaRequest::new_from_response(&secret, &response).unwrap();

        assert_eq!(&secret, &request.secret().as_str());
        assert_eq!(&response, &request.captcha.response().as_str());
        assert_none!(&request.captcha.user_ip());
        assert_none!(&request.captcha.site_key());
    }
}
