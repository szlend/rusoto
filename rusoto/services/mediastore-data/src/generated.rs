// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteObjectRequest {
    /// <p>The path (including the file name) where the object is stored in the container. Format: &lt;folder name&gt;/&lt;folder name&gt;/&lt;file name&gt;</p>
    #[serde(rename = "Path")]
    pub path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteObjectResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeObjectRequest {
    /// <p>The path (including the file name) where the object is stored in the container. Format: &lt;folder name&gt;/&lt;folder name&gt;/&lt;file name&gt;</p>
    #[serde(rename = "Path")]
    pub path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeObjectResponse {
    /// <p>An optional <code>CacheControl</code> header that allows the caller to control the object's cache behavior. Headers can be passed in as specified in the HTTP at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p> <p>Headers with a custom user-defined value are also accepted.</p>
    #[serde(rename = "CacheControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<String>,
    /// <p>The length of the object in bytes.</p>
    #[serde(rename = "ContentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    /// <p>The content type of the object.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The ETag that represents a unique instance of the object.</p>
    #[serde(rename = "ETag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    /// <p>The date and time that the object was last modified.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetObjectRequest {
    /// <p>The path (including the file name) where the object is stored in the container. Format: &lt;folder name&gt;/&lt;folder name&gt;/&lt;file name&gt;</p> <p>For example, to upload the file <code>mlaw.avi</code> to the folder path <code>premium\canada</code> in the container <code>movies</code>, enter the path <code>premium/canada/mlaw.avi</code>.</p> <p>Do not include the container name in this path.</p> <p>If the path includes any folders that don't exist yet, the service creates them. For example, suppose you have an existing <code>premium/usa</code> subfolder. If you specify <code>premium/canada</code>, the service creates a <code>canada</code> subfolder in the <code>premium</code> folder. You then have two subfolders, <code>usa</code> and <code>canada</code>, in the <code>premium</code> folder. </p> <p>There is no correlation between the path to the source and the path (folders) in the container in AWS Elemental MediaStore.</p> <p>For more information about folders and how they exist in a container, see the <a href="http://docs.aws.amazon.com/mediastore/latest/ug/">AWS Elemental MediaStore User Guide</a>.</p> <p>The file name is the name that is assigned to the file that you upload. The file can have the same name inside and outside of AWS Elemental MediaStore, or it can have the same name. The file name can include or omit an extension. </p>
    #[serde(rename = "Path")]
    pub path: String,
    /// <p>The range bytes of an object to retrieve. For more information about the <code>Range</code> header, go to <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35</a>.</p>
    #[serde(rename = "Range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectResponse {
    /// <p>The bytes of the object. </p>
    pub body: Option<Vec<u8>>,
    /// <p>An optional <code>CacheControl</code> header that allows the caller to control the object's cache behavior. Headers can be passed in as specified in the HTTP spec at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p> <p>Headers with a custom user-defined value are also accepted.</p>
    pub cache_control: Option<String>,
    /// <p>The length of the object in bytes.</p>
    pub content_length: Option<i64>,
    /// <p>The range of bytes to retrieve.</p>
    pub content_range: Option<String>,
    /// <p>The content type of the object.</p>
    pub content_type: Option<String>,
    /// <p>The ETag that represents a unique instance of the object.</p>
    pub e_tag: Option<String>,
    /// <p>The date and time that the object was last modified.</p>
    pub last_modified: Option<f64>,
    /// <p>The HTML status code of the request. Status codes ranging from 200 to 299 indicate success. All other status codes indicate the type of error that occurred.</p>
    pub status_code: i64,
}

/// <p>A metadata entry for a folder or object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Item {
    /// <p>The length of the item in bytes.</p>
    #[serde(rename = "ContentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    /// <p>The content type of the item.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The ETag that represents a unique instance of the item.</p>
    #[serde(rename = "ETag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    /// <p>The date and time that the item was last modified.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>The name of the item.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The item type (folder or object).</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListItemsRequest {
    /// <p>The maximum results to return. The service might return fewer results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>NextToken</code> received in the <code>ListItemsResponse</code> for the same container and path. Tokens expire after 15 minutes.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The path in the container from which to retrieve items. Format: &lt;folder name&gt;/&lt;folder name&gt;/&lt;file name&gt;</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListItemsResponse {
    /// <p>Metadata entries for the folders and objects at the requested path.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Item>>,
    /// <p>The <code>NextToken</code> used to request the next page of results using <code>ListItems</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutObjectRequest {
    /// <p>The bytes to be stored. </p>
    #[serde(rename = "Body")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub body: Vec<u8>,
    /// <p>An optional <code>CacheControl</code> header that allows the caller to control the object's cache behavior. Headers can be passed in as specified in the HTTP at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p> <p>Headers with a custom user-defined value are also accepted.</p>
    #[serde(rename = "CacheControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<String>,
    /// <p>The content type of the object.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The path (including the file name) where the object is stored in the container. Format: &lt;folder name&gt;/&lt;folder name&gt;/&lt;file name&gt;</p> <p>For example, to upload the file <code>mlaw.avi</code> to the folder path <code>premium\canada</code> in the container <code>movies</code>, enter the path <code>premium/canada/mlaw.avi</code>.</p> <p>Do not include the container name in this path.</p> <p>If the path includes any folders that don't exist yet, the service creates them. For example, suppose you have an existing <code>premium/usa</code> subfolder. If you specify <code>premium/canada</code>, the service creates a <code>canada</code> subfolder in the <code>premium</code> folder. You then have two subfolders, <code>usa</code> and <code>canada</code>, in the <code>premium</code> folder. </p> <p>There is no correlation between the path to the source and the path (folders) in the container in AWS Elemental MediaStore.</p> <p>For more information about folders and how they exist in a container, see the <a href="http://docs.aws.amazon.com/mediastore/latest/ug/">AWS Elemental MediaStore User Guide</a>.</p> <p>The file name is the name that is assigned to the file that you upload. The file can have the same name inside and outside of AWS Elemental MediaStore, or it can have the same name. The file name can include or omit an extension. </p>
    #[serde(rename = "Path")]
    pub path: String,
    /// <p>Indicates the storage class of a <code>Put</code> request. Defaults to high-performance temporal storage class, and objects are persisted into durable storage shortly after being received.</p>
    #[serde(rename = "StorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutObjectResponse {
    /// <p>The SHA256 digest of the object that is persisted.</p>
    #[serde(rename = "ContentSHA256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_sha256: Option<String>,
    /// <p>Unique identifier of the object in the container.</p>
    #[serde(rename = "ETag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    /// <p>The storage class where the object was persisted. Should be “Temporal”.</p>
    #[serde(rename = "StorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

/// Errors returned by DeleteObject
#[derive(Debug, PartialEq)]
pub enum DeleteObjectError {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>Could not perform an operation on an object that does not exist.</p>
    ObjectNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteObjectError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteObjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ContainerNotFoundException" => {
                    return DeleteObjectError::ContainerNotFound(String::from(error_message))
                }
                "InternalServerError" => {
                    return DeleteObjectError::InternalServerError(String::from(error_message))
                }
                "ObjectNotFoundException" => {
                    return DeleteObjectError::ObjectNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteObjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteObjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteObjectError {
    fn from(err: serde_json::error::Error) -> DeleteObjectError {
        DeleteObjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteObjectError {
    fn from(err: CredentialsError) -> DeleteObjectError {
        DeleteObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteObjectError {
    fn from(err: HttpDispatchError) -> DeleteObjectError {
        DeleteObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteObjectError {
    fn from(err: io::Error) -> DeleteObjectError {
        DeleteObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteObjectError {
    fn description(&self) -> &str {
        match *self {
            DeleteObjectError::ContainerNotFound(ref cause) => cause,
            DeleteObjectError::InternalServerError(ref cause) => cause,
            DeleteObjectError::ObjectNotFound(ref cause) => cause,
            DeleteObjectError::Validation(ref cause) => cause,
            DeleteObjectError::Credentials(ref err) => err.description(),
            DeleteObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteObjectError::ParseError(ref cause) => cause,
            DeleteObjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeObject
#[derive(Debug, PartialEq)]
pub enum DescribeObjectError {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>Could not perform an operation on an object that does not exist.</p>
    ObjectNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeObjectError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeObjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ContainerNotFoundException" => {
                    return DescribeObjectError::ContainerNotFound(String::from(error_message))
                }
                "InternalServerError" => {
                    return DescribeObjectError::InternalServerError(String::from(error_message))
                }
                "ObjectNotFoundException" => {
                    return DescribeObjectError::ObjectNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeObjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeObjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeObjectError {
    fn from(err: serde_json::error::Error) -> DescribeObjectError {
        DescribeObjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeObjectError {
    fn from(err: CredentialsError) -> DescribeObjectError {
        DescribeObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeObjectError {
    fn from(err: HttpDispatchError) -> DescribeObjectError {
        DescribeObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeObjectError {
    fn from(err: io::Error) -> DescribeObjectError {
        DescribeObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeObjectError {
    fn description(&self) -> &str {
        match *self {
            DescribeObjectError::ContainerNotFound(ref cause) => cause,
            DescribeObjectError::InternalServerError(ref cause) => cause,
            DescribeObjectError::ObjectNotFound(ref cause) => cause,
            DescribeObjectError::Validation(ref cause) => cause,
            DescribeObjectError::Credentials(ref err) => err.description(),
            DescribeObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeObjectError::ParseError(ref cause) => cause,
            DescribeObjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetObject
#[derive(Debug, PartialEq)]
pub enum GetObjectError {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>Could not perform an operation on an object that does not exist.</p>
    ObjectNotFound(String),
    /// <p>The requested content range is not valid.</p>
    RequestedRangeNotSatisfiable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetObjectError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetObjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ContainerNotFoundException" => {
                    return GetObjectError::ContainerNotFound(String::from(error_message))
                }
                "InternalServerError" => {
                    return GetObjectError::InternalServerError(String::from(error_message))
                }
                "ObjectNotFoundException" => {
                    return GetObjectError::ObjectNotFound(String::from(error_message))
                }
                "RequestedRangeNotSatisfiableException" => {
                    return GetObjectError::RequestedRangeNotSatisfiable(String::from(error_message))
                }
                "ValidationException" => {
                    return GetObjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetObjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetObjectError {
    fn from(err: serde_json::error::Error) -> GetObjectError {
        GetObjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetObjectError {
    fn from(err: CredentialsError) -> GetObjectError {
        GetObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetObjectError {
    fn from(err: HttpDispatchError) -> GetObjectError {
        GetObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetObjectError {
    fn from(err: io::Error) -> GetObjectError {
        GetObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectError {
    fn description(&self) -> &str {
        match *self {
            GetObjectError::ContainerNotFound(ref cause) => cause,
            GetObjectError::InternalServerError(ref cause) => cause,
            GetObjectError::ObjectNotFound(ref cause) => cause,
            GetObjectError::RequestedRangeNotSatisfiable(ref cause) => cause,
            GetObjectError::Validation(ref cause) => cause,
            GetObjectError::Credentials(ref err) => err.description(),
            GetObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetObjectError::ParseError(ref cause) => cause,
            GetObjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListItems
#[derive(Debug, PartialEq)]
pub enum ListItemsError {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListItemsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListItemsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ContainerNotFoundException" => {
                    return ListItemsError::ContainerNotFound(String::from(error_message))
                }
                "InternalServerError" => {
                    return ListItemsError::InternalServerError(String::from(error_message))
                }
                "ValidationException" => {
                    return ListItemsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListItemsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListItemsError {
    fn from(err: serde_json::error::Error) -> ListItemsError {
        ListItemsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListItemsError {
    fn from(err: CredentialsError) -> ListItemsError {
        ListItemsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListItemsError {
    fn from(err: HttpDispatchError) -> ListItemsError {
        ListItemsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListItemsError {
    fn from(err: io::Error) -> ListItemsError {
        ListItemsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListItemsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListItemsError {
    fn description(&self) -> &str {
        match *self {
            ListItemsError::ContainerNotFound(ref cause) => cause,
            ListItemsError::InternalServerError(ref cause) => cause,
            ListItemsError::Validation(ref cause) => cause,
            ListItemsError::Credentials(ref err) => err.description(),
            ListItemsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListItemsError::ParseError(ref cause) => cause,
            ListItemsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutObject
#[derive(Debug, PartialEq)]
pub enum PutObjectError {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutObjectError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> PutObjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ContainerNotFoundException" => {
                    return PutObjectError::ContainerNotFound(String::from(error_message))
                }
                "InternalServerError" => {
                    return PutObjectError::InternalServerError(String::from(error_message))
                }
                "ValidationException" => {
                    return PutObjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PutObjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutObjectError {
    fn from(err: serde_json::error::Error) -> PutObjectError {
        PutObjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutObjectError {
    fn from(err: CredentialsError) -> PutObjectError {
        PutObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutObjectError {
    fn from(err: HttpDispatchError) -> PutObjectError {
        PutObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutObjectError {
    fn from(err: io::Error) -> PutObjectError {
        PutObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutObjectError {
    fn description(&self) -> &str {
        match *self {
            PutObjectError::ContainerNotFound(ref cause) => cause,
            PutObjectError::InternalServerError(ref cause) => cause,
            PutObjectError::Validation(ref cause) => cause,
            PutObjectError::Credentials(ref err) => err.description(),
            PutObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutObjectError::ParseError(ref cause) => cause,
            PutObjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the MediaStore Data API. MediaStore Data clients implement this trait.
pub trait MediaStoreData {
    /// <p>Deletes an object at the specified path.</p>
    fn delete_object(
        &self,
        input: DeleteObjectRequest,
    ) -> RusotoFuture<DeleteObjectResponse, DeleteObjectError>;

    /// <p>Gets the headers for an object at the specified path.</p>
    fn describe_object(
        &self,
        input: DescribeObjectRequest,
    ) -> RusotoFuture<DescribeObjectResponse, DescribeObjectError>;

    /// <p>Downloads the object at the specified path.</p>
    fn get_object(
        &self,
        input: GetObjectRequest,
    ) -> RusotoFuture<GetObjectResponse, GetObjectError>;

    /// <p>Provides a list of metadata entries about folders and objects in the specified folder.</p>
    fn list_items(
        &self,
        input: ListItemsRequest,
    ) -> RusotoFuture<ListItemsResponse, ListItemsError>;

    /// <p>Uploads an object to the specified path. Object sizes are limited to 10 MB.</p>
    fn put_object(
        &self,
        input: PutObjectRequest,
    ) -> RusotoFuture<PutObjectResponse, PutObjectError>;
}
/// A client for the MediaStore Data API.
pub struct MediaStoreDataClient {
    client: Client,
    region: region::Region,
}

impl MediaStoreDataClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MediaStoreDataClient {
        MediaStoreDataClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaStoreDataClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        MediaStoreDataClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl MediaStoreData for MediaStoreDataClient {
    /// <p>Deletes an object at the specified path.</p>
    fn delete_object(
        &self,
        input: DeleteObjectRequest,
    ) -> RusotoFuture<DeleteObjectResponse, DeleteObjectError> {
        let request_uri = format!("/{path}", path = input.path);

        let mut request = SignedRequest::new("DELETE", "mediastore", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.mediastore".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteObjectResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteObjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the headers for an object at the specified path.</p>
    fn describe_object(
        &self,
        input: DescribeObjectRequest,
    ) -> RusotoFuture<DescribeObjectResponse, DescribeObjectError> {
        let request_uri = format!("/{path}", path = input.path);

        let mut request = SignedRequest::new("HEAD", "mediastore", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.mediastore".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<DescribeObjectResponse>(&body).unwrap();

                    if let Some(cache_control) = response.headers.get("Cache-Control") {
                        let value = cache_control.to_owned();
                        result.cache_control = Some(value)
                    };
                    if let Some(content_length) = response.headers.get("Content-Length") {
                        let value = content_length.to_owned();
                        result.content_length = Some(value.parse::<i64>().unwrap())
                    };
                    if let Some(content_type) = response.headers.get("Content-Type") {
                        let value = content_type.to_owned();
                        result.content_type = Some(value)
                    };
                    if let Some(e_tag) = response.headers.get("ETag") {
                        let value = e_tag.to_owned();
                        result.e_tag = Some(value)
                    };
                    if let Some(last_modified) = response.headers.get("Last-Modified") {
                        let value = last_modified.to_owned();
                        result.last_modified = Some(value)
                    };

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeObjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Downloads the object at the specified path.</p>
    fn get_object(
        &self,
        input: GetObjectRequest,
    ) -> RusotoFuture<GetObjectResponse, GetObjectError> {
        let request_uri = format!("/{path}", path = input.path);

        let mut request = SignedRequest::new("GET", "mediastore", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.mediastore".to_string());

        if let Some(ref range) = input.range {
            request.add_header("Range", &range.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut result = GetObjectResponse::default();
                    result.body = response.body;

                    if let Some(cache_control) = response.headers.get("Cache-Control") {
                        let value = cache_control.to_owned();
                        result.cache_control = Some(value)
                    };
                    if let Some(content_length) = response.headers.get("Content-Length") {
                        let value = content_length.to_owned();
                        result.content_length = Some(value.parse::<i64>().unwrap())
                    };
                    if let Some(content_range) = response.headers.get("Content-Range") {
                        let value = content_range.to_owned();
                        result.content_range = Some(value)
                    };
                    if let Some(content_type) = response.headers.get("Content-Type") {
                        let value = content_type.to_owned();
                        result.content_type = Some(value)
                    };
                    if let Some(e_tag) = response.headers.get("ETag") {
                        let value = e_tag.to_owned();
                        result.e_tag = Some(value)
                    };
                    if let Some(last_modified) = response.headers.get("Last-Modified") {
                        let value = last_modified.to_owned();
                        result.last_modified = Some(value)
                    };
                    result.status_code = Some(response.status.as_u16());
                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetObjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Provides a list of metadata entries about folders and objects in the specified folder.</p>
    fn list_items(
        &self,
        input: ListItemsRequest,
    ) -> RusotoFuture<ListItemsResponse, ListItemsError> {
        let request_uri = "/";

        let mut request = SignedRequest::new("GET", "mediastore", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.mediastore".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.path {
            params.put("Path", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListItemsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListItemsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Uploads an object to the specified path. Object sizes are limited to 10 MB.</p>
    fn put_object(
        &self,
        input: PutObjectRequest,
    ) -> RusotoFuture<PutObjectResponse, PutObjectError> {
        let request_uri = format!("/{path}", path = input.path);

        let mut request = SignedRequest::new("PUT", "mediastore", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.mediastore".to_string());
        let encoded = Some(input.body.to_owned());
        request.set_payload(encoded);

        if let Some(ref cache_control) = input.cache_control {
            request.add_header("Cache-Control", &cache_control.to_string());
        }

        if let Some(ref content_type) = input.content_type {
            request.add_header("Content-Type", &content_type.to_string());
        }

        if let Some(ref storage_class) = input.storage_class {
            request.add_header("x-amz-storage-class", &storage_class.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<PutObjectResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutObjectError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
