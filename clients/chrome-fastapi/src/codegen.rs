#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{RequestBuilderExt, encode_path};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
	/// Error types.
	pub mod error {
		/// Error from a TryFrom or FromStr implementation.
		pub struct ConversionError(::std::borrow::Cow<'static, str>);
		impl ::std::error::Error for ConversionError {}
		impl ::std::fmt::Display for ConversionError {
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
				::std::fmt::Display::fmt(&self.0, f)
			}
		}
		impl ::std::fmt::Debug for ConversionError {
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
				::std::fmt::Debug::fmt(&self.0, f)
			}
		}
		impl From<&'static str> for ConversionError {
			fn from(value: &'static str) -> Self {
				Self(value.into())
			}
		}
		impl From<String> for ConversionError {
			fn from(value: String) -> Self {
				Self(value.into())
			}
		}
	}
	///HtmlResponse
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "title": "HtmlResponse",
	///  "type": "object",
	///  "required": [
	///    "html",
	///    "url"
	///  ],
	///  "properties": {
	///    "html": {
	///      "title": "Html",
	///      "type": "string"
	///    },
	///    "url": {
	///      "title": "Url",
	///      "type": "string",
	///      "format": "uri",
	///      "maxLength": 2083,
	///      "minLength": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(Clone, Debug, ::serde::Deserialize, ::serde::Serialize)]
	pub struct HtmlResponse {
		pub html: ::std::string::String,
		pub url: ::std::string::String,
	}
	impl ::std::convert::From<&HtmlResponse> for HtmlResponse {
		fn from(value: &HtmlResponse) -> Self {
			value.clone()
		}
	}
	///HttpValidationError
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "title": "HTTPValidationError",
	///  "type": "object",
	///  "properties": {
	///    "detail": {
	///      "title": "Detail",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ValidationError"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(Clone, Debug, ::serde::Deserialize, ::serde::Serialize)]
	pub struct HttpValidationError {
		#[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
		pub detail: ::std::vec::Vec<ValidationError>,
	}
	impl ::std::convert::From<&HttpValidationError> for HttpValidationError {
		fn from(value: &HttpValidationError) -> Self {
			value.clone()
		}
	}
	impl ::std::default::Default for HttpValidationError {
		fn default() -> Self {
			Self { detail: Default::default() }
		}
	}
	///LocationItem
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "type": "string"
	///    },
	///    {
	///      "type": "integer"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(Clone, Debug, ::serde::Deserialize, ::serde::Serialize)]
	#[serde(untagged)]
	pub enum LocationItem {
		Variant0(::std::string::String),
		Variant1(i64),
	}
	impl ::std::convert::From<&Self> for LocationItem {
		fn from(value: &LocationItem) -> Self {
			value.clone()
		}
	}
	impl ::std::str::FromStr for LocationItem {
		type Err = self::error::ConversionError;

		fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
			let Ok(v) = value.parse();
			Ok(Self::Variant0(v))
		}
	}
	impl ::std::convert::TryFrom<&str> for LocationItem {
		type Error = self::error::ConversionError;

		fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}
	impl ::std::convert::TryFrom<&::std::string::String> for LocationItem {
		type Error = self::error::ConversionError;

		fn try_from(value: &::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}
	impl ::std::convert::TryFrom<::std::string::String> for LocationItem {
		type Error = self::error::ConversionError;

		fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}
	impl ::std::fmt::Display for LocationItem {
		fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
			match self {
				Self::Variant0(x) => x.fmt(f),
				Self::Variant1(x) => x.fmt(f),
			}
		}
	}
	impl ::std::convert::From<i64> for LocationItem {
		fn from(value: i64) -> Self {
			Self::Variant1(value)
		}
	}
	///UrlRequest
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "title": "UrlRequest",
	///  "examples": [
	///    {
	///      "url": "https://example.com"
	///    }
	///  ],
	///  "type": "object",
	///  "required": [
	///    "url"
	///  ],
	///  "properties": {
	///    "url": {
	///      "title": "Url",
	///      "type": "string",
	///      "format": "uri",
	///      "maxLength": 2083,
	///      "minLength": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(Clone, Debug, ::serde::Deserialize, ::serde::Serialize)]
	pub struct UrlRequest {
		pub url: ::std::string::String,
	}
	impl ::std::convert::From<&UrlRequest> for UrlRequest {
		fn from(value: &UrlRequest) -> Self {
			value.clone()
		}
	}
	///ValidationError
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "title": "ValidationError",
	///  "type": "object",
	///  "required": [
	///    "loc",
	///    "msg",
	///    "type"
	///  ],
	///  "properties": {
	///    "loc": {
	///      "title": "Location",
	///      "type": "array",
	///      "items": {
	///        "anyOf": [
	///          {
	///            "type": "string"
	///          },
	///          {
	///            "type": "integer"
	///          }
	///        ]
	///      }
	///    },
	///    "msg": {
	///      "title": "Message",
	///      "type": "string"
	///    },
	///    "type": {
	///      "title": "Error Type",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(Clone, Debug, ::serde::Deserialize, ::serde::Serialize)]
	pub struct ValidationError {
		pub loc: ::std::vec::Vec<LocationItem>,
		pub msg: ::std::string::String,
		#[serde(rename = "type")]
		pub type_: ::std::string::String,
	}
	impl ::std::convert::From<&ValidationError> for ValidationError {
		fn from(value: &ValidationError) -> Self {
			value.clone()
		}
	}
}
#[derive(Clone, Debug)]
/**Client for Web Scraper API

Simple web scraping API using Selenium for content fetching

Version: 1.0.0*/
pub struct Client {
	pub(crate) baseurl: String,
	pub(crate) client: reqwest::Client,
}
impl Client {
	/// Create a new client.
	///
	/// `baseurl` is the base URL provided to the internal
	/// `reqwest::Client`, and should include a scheme and hostname,
	/// as well as port and a path stem if applicable.
	pub fn new(baseurl: &str) -> Self {
		#[cfg(not(target_arch = "wasm32"))]
		let client = {
			let dur = std::time::Duration::from_secs(15);
			reqwest::ClientBuilder::new().connect_timeout(dur).timeout(dur)
		};
		#[cfg(target_arch = "wasm32")]
		let client = reqwest::ClientBuilder::new();
		Self::new_with_client(baseurl, client.build().unwrap())
	}

	/// Construct a new client with an existing `reqwest::Client`,
	/// allowing more control over its configuration.
	///
	/// `baseurl` is the base URL provided to the internal
	/// `reqwest::Client`, and should include a scheme and hostname,
	/// as well as port and a path stem if applicable.
	pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
		Self { baseurl: baseurl.to_string(), client }
	}

	/// Get the base URL to which requests are made.
	pub fn baseurl(&self) -> &String {
		&self.baseurl
	}

	/// Get the internal `reqwest::Client` used to make requests.
	pub fn client(&self) -> &reqwest::Client {
		&self.client
	}

	/// Get the version of this API.
	///
	/// This string is pulled directly from the source OpenAPI
	/// document and may be in any format the API selects.
	pub fn api_version(&self) -> &'static str {
		"1.0.0"
	}
}
#[allow(clippy::all)]
#[allow(elided_named_lifetimes)]
impl Client {
	/**Fetch HTML content from a URL

	Sends a `POST` request to `/fetch-html`

	*/
	pub async fn fetch_html_fetch_html_post<'a>(
		&'a self,
		body: &'a types::UrlRequest,
	) -> Result<ResponseValue<types::HtmlResponse>, Error<types::HttpValidationError>> {
		let url = format!("{}/fetch-html", self.baseurl,);
		#[allow(unused_mut)]
		let mut request = self.client.post(url).header(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("application/json")).json(&body).build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			422u16 => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	/**Health Check

	Sends a `GET` request to `/health`

	*/
	pub async fn health_check_health_get<'a>(&'a self) -> Result<ResponseValue<::serde_json::Value>, Error<()>> {
		let url = format!("{}/health", self.baseurl,);
		#[allow(unused_mut)]
		let mut request = self.client.get(url).header(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("application/json")).build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
	#[allow(unused_imports)]
	pub use super::Client;
}
