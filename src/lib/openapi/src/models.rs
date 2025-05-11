#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetDisplayHeaderParams {
        pub access_token: Option<String>,
        pub id: Option<String>,
        pub refresh_rate: Option<String>,
        pub battery_voltage: Option<String>,
        pub fw_version: Option<String>,
        pub rssi: Option<String>,
    }

            
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetSetupHeaderParams {
        pub id: Option<String>,
        pub access_token: Option<String>,
    }

            
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostLogHeaderParams {
        pub id: Option<String>,
        pub access_token: Option<String>,
        pub refresh_rate: Option<String>,
        pub battery_voltage: Option<String>,
        pub fw_version: Option<String>,
        pub rssi: Option<String>,
    }

            


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeviceInfo {
    /// The API key of the device.
    #[serde(rename = "api_key")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub api_key: Option<Nullable<String>>,

    /// A human-readable ID for the device.
    #[serde(rename = "friendly_id")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub friendly_id: Option<Nullable<String>>,

    /// URL for a device image.
    #[serde(rename = "image_url")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_url: Option<Nullable<String>>,

    /// Filename of a device image.
    #[serde(rename = "filename")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filename: Option<Nullable<String>>,

    /// A message indicating the outcome of the request.
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    /// Flag indicating if a firmware reset is needed.
    #[serde(rename = "reset_firmware")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reset_firmware: Option<Nullable<bool>>,

    /// Error message.
    #[serde(rename = "error")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<Nullable<String>>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<i32>,

}





impl DeviceInfo {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> DeviceInfo {
        DeviceInfo {
            api_key: None,
            friendly_id: None,
            image_url: None,
            filename: None,
            message: None,
            reset_firmware: None,
            error: None,
            status: None,
        }
    }
}

/// Converts the DeviceInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.api_key.as_ref().map(|api_key| {
                [
                    "api_key".to_string(),
                    api_key.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),


            self.friendly_id.as_ref().map(|friendly_id| {
                [
                    "friendly_id".to_string(),
                    friendly_id.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),


            self.image_url.as_ref().map(|image_url| {
                [
                    "image_url".to_string(),
                    image_url.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),


            self.filename.as_ref().map(|filename| {
                [
                    "filename".to_string(),
                    filename.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),


            self.reset_firmware.as_ref().map(|reset_firmware| {
                [
                    "reset_firmware".to_string(),
                    reset_firmware.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),


            self.error.as_ref().map(|error| {
                [
                    "error".to_string(),
                    error.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),


            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DeviceInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DeviceInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub api_key: Vec<String>,
            pub friendly_id: Vec<String>,
            pub image_url: Vec<String>,
            pub filename: Vec<String>,
            pub message: Vec<String>,
            pub reset_firmware: Vec<bool>,
            pub error: Vec<String>,
            pub status: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DeviceInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "api_key" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DeviceInfo".to_string()),
                    "friendly_id" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DeviceInfo".to_string()),
                    "image_url" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DeviceInfo".to_string()),
                    "filename" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DeviceInfo".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "reset_firmware" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DeviceInfo".to_string()),
                    "error" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DeviceInfo".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DeviceInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DeviceInfo {
            api_key: std::result::Result::Err("Nullable types not supported in DeviceInfo".to_string())?,
            friendly_id: std::result::Result::Err("Nullable types not supported in DeviceInfo".to_string())?,
            image_url: std::result::Result::Err("Nullable types not supported in DeviceInfo".to_string())?,
            filename: std::result::Result::Err("Nullable types not supported in DeviceInfo".to_string())?,
            message: intermediate_rep.message.into_iter().next(),
            reset_firmware: std::result::Result::Err("Nullable types not supported in DeviceInfo".to_string())?,
            error: std::result::Result::Err("Nullable types not supported in DeviceInfo".to_string())?,
            status: intermediate_rep.status.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DeviceInfo> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<DeviceInfo>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DeviceInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DeviceInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<DeviceInfo> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DeviceInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DeviceInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DisplayInfo {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<i32>,

    /// URL of the image to display.
    #[serde(rename = "image_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_url: Option<String>,

    /// Filename of the image.
    #[serde(rename = "filename")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filename: Option<String>,

    /// The refresh rate for the device in seconds (multiplied by 3 internally).
    #[serde(rename = "refresh_rate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub refresh_rate: Option<i32>,

    #[serde(rename = "reset_firmware")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reset_firmware: Option<bool>,

    #[serde(rename = "update_firmware")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub update_firmware: Option<bool>,

    #[serde(rename = "firmware_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub firmware_url: Option<String>,

    #[serde(rename = "special_function")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub special_function: Option<String>,

}





impl DisplayInfo {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> DisplayInfo {
        DisplayInfo {
            status: None,
            image_url: None,
            filename: None,
            refresh_rate: None,
            reset_firmware: None,
            update_firmware: None,
            firmware_url: None,
            special_function: None,
        }
    }
}

/// Converts the DisplayInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for DisplayInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),


            self.image_url.as_ref().map(|image_url| {
                [
                    "image_url".to_string(),
                    image_url.to_string(),
                ].join(",")
            }),


            self.filename.as_ref().map(|filename| {
                [
                    "filename".to_string(),
                    filename.to_string(),
                ].join(",")
            }),


            self.refresh_rate.as_ref().map(|refresh_rate| {
                [
                    "refresh_rate".to_string(),
                    refresh_rate.to_string(),
                ].join(",")
            }),


            self.reset_firmware.as_ref().map(|reset_firmware| {
                [
                    "reset_firmware".to_string(),
                    reset_firmware.to_string(),
                ].join(",")
            }),


            self.update_firmware.as_ref().map(|update_firmware| {
                [
                    "update_firmware".to_string(),
                    update_firmware.to_string(),
                ].join(",")
            }),


            self.firmware_url.as_ref().map(|firmware_url| {
                [
                    "firmware_url".to_string(),
                    firmware_url.to_string(),
                ].join(",")
            }),


            self.special_function.as_ref().map(|special_function| {
                [
                    "special_function".to_string(),
                    special_function.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DisplayInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DisplayInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub status: Vec<i32>,
            pub image_url: Vec<String>,
            pub filename: Vec<String>,
            pub refresh_rate: Vec<i32>,
            pub reset_firmware: Vec<bool>,
            pub update_firmware: Vec<bool>,
            pub firmware_url: Vec<String>,
            pub special_function: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DisplayInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "image_url" => intermediate_rep.image_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "filename" => intermediate_rep.filename.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "refresh_rate" => intermediate_rep.refresh_rate.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "reset_firmware" => intermediate_rep.reset_firmware.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "update_firmware" => intermediate_rep.update_firmware.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "firmware_url" => intermediate_rep.firmware_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "special_function" => intermediate_rep.special_function.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DisplayInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DisplayInfo {
            status: intermediate_rep.status.into_iter().next(),
            image_url: intermediate_rep.image_url.into_iter().next(),
            filename: intermediate_rep.filename.into_iter().next(),
            refresh_rate: intermediate_rep.refresh_rate.into_iter().next(),
            reset_firmware: intermediate_rep.reset_firmware.into_iter().next(),
            update_firmware: intermediate_rep.update_firmware.into_iter().next(),
            firmware_url: intermediate_rep.firmware_url.into_iter().next(),
            special_function: intermediate_rep.special_function.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DisplayInfo> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<DisplayInfo>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DisplayInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DisplayInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<DisplayInfo> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DisplayInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DisplayInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ErrorResponse {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<i32>,

    /// Error message.
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "error")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<Nullable<String>>,

}





impl ErrorResponse {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ErrorResponse {
        ErrorResponse {
            status: None,
            message: None,
            error: None,
        }
    }
}

/// Converts the ErrorResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),


            self.error.as_ref().map(|error| {
                [
                    "error".to_string(),
                    error.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ErrorResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ErrorResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub status: Vec<i32>,
            pub message: Vec<String>,
            pub error: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ErrorResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "error" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ErrorResponse".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ErrorResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ErrorResponse {
            status: intermediate_rep.status.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            error: std::result::Result::Err("Nullable types not supported in ErrorResponse".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ErrorResponse> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ErrorResponse>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ErrorResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ErrorResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ErrorResponse> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ErrorResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ErrorResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetDisplay500Response {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<i32>,

    /// Error message.
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "error")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<Nullable<String>>,

    #[serde(rename = "reset_firmware")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reset_firmware: Option<bool>,

    /// URL of the 'not found' image.
    #[serde(rename = "image_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_url: Option<String>,

    /// Filename of the 'not found' image.
    #[serde(rename = "filename")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filename: Option<String>,

}





impl GetDisplay500Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> GetDisplay500Response {
        GetDisplay500Response {
            status: None,
            message: None,
            error: None,
            reset_firmware: None,
            image_url: None,
            filename: None,
        }
    }
}

/// Converts the GetDisplay500Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for GetDisplay500Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),


            self.error.as_ref().map(|error| {
                [
                    "error".to_string(),
                    error.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),


            self.reset_firmware.as_ref().map(|reset_firmware| {
                [
                    "reset_firmware".to_string(),
                    reset_firmware.to_string(),
                ].join(",")
            }),


            self.image_url.as_ref().map(|image_url| {
                [
                    "image_url".to_string(),
                    image_url.to_string(),
                ].join(",")
            }),


            self.filename.as_ref().map(|filename| {
                [
                    "filename".to_string(),
                    filename.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetDisplay500Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetDisplay500Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub status: Vec<i32>,
            pub message: Vec<String>,
            pub error: Vec<String>,
            pub reset_firmware: Vec<bool>,
            pub image_url: Vec<String>,
            pub filename: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetDisplay500Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "error" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in GetDisplay500Response".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "reset_firmware" => intermediate_rep.reset_firmware.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "image_url" => intermediate_rep.image_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "filename" => intermediate_rep.filename.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetDisplay500Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetDisplay500Response {
            status: intermediate_rep.status.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            error: std::result::Result::Err("Nullable types not supported in GetDisplay500Response".to_string())?,
            reset_firmware: intermediate_rep.reset_firmware.into_iter().next(),
            image_url: intermediate_rep.image_url.into_iter().next(),
            filename: intermediate_rep.filename.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetDisplay500Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<GetDisplay500Response>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetDisplay500Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetDisplay500Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<GetDisplay500Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetDisplay500Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetDisplay500Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LogEntry {
    /// Unix timestamp of the log entry creation.
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: i32,

    #[serde(rename = "message")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<Nullable<String>>,

    #[serde(rename = "level")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub level: Option<Nullable<String>>,

    #[serde(rename = "device_status")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_status: Option<Nullable<String>>,

    #[serde(rename = "battery_voltage")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub battery_voltage: Option<Nullable<f64>>,

    #[serde(rename = "rssi")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rssi: Option<Nullable<i32>>,

    #[serde(rename = "firmware_version")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub firmware_version: Option<Nullable<String>>,

}





impl LogEntry {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(creation_timestamp: i32, ) -> LogEntry {
        LogEntry {
            creation_timestamp,
            message: None,
            level: None,
            device_status: None,
            battery_voltage: None,
            rssi: None,
            firmware_version: None,
        }
    }
}

/// Converts the LogEntry value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for LogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("creation_timestamp".to_string()),
            Some(self.creation_timestamp.to_string()),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),


            self.level.as_ref().map(|level| {
                [
                    "level".to_string(),
                    level.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),


            self.device_status.as_ref().map(|device_status| {
                [
                    "device_status".to_string(),
                    device_status.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),


            self.battery_voltage.as_ref().map(|battery_voltage| {
                [
                    "battery_voltage".to_string(),
                    battery_voltage.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),


            self.rssi.as_ref().map(|rssi| {
                [
                    "rssi".to_string(),
                    rssi.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),


            self.firmware_version.as_ref().map(|firmware_version| {
                [
                    "firmware_version".to_string(),
                    firmware_version.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LogEntry value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LogEntry {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub creation_timestamp: Vec<i32>,
            pub message: Vec<String>,
            pub level: Vec<String>,
            pub device_status: Vec<String>,
            pub battery_voltage: Vec<f64>,
            pub rssi: Vec<i32>,
            pub firmware_version: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LogEntry".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "creation_timestamp" => intermediate_rep.creation_timestamp.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "message" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in LogEntry".to_string()),
                    "level" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in LogEntry".to_string()),
                    "device_status" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in LogEntry".to_string()),
                    "battery_voltage" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in LogEntry".to_string()),
                    "rssi" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in LogEntry".to_string()),
                    "firmware_version" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in LogEntry".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing LogEntry".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LogEntry {
            creation_timestamp: intermediate_rep.creation_timestamp.into_iter().next().ok_or_else(|| "creation_timestamp missing in LogEntry".to_string())?,
            message: std::result::Result::Err("Nullable types not supported in LogEntry".to_string())?,
            level: std::result::Result::Err("Nullable types not supported in LogEntry".to_string())?,
            device_status: std::result::Result::Err("Nullable types not supported in LogEntry".to_string())?,
            battery_voltage: std::result::Result::Err("Nullable types not supported in LogEntry".to_string())?,
            rssi: std::result::Result::Err("Nullable types not supported in LogEntry".to_string())?,
            firmware_version: std::result::Result::Err("Nullable types not supported in LogEntry".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LogEntry> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<LogEntry>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LogEntry>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LogEntry - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<LogEntry> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LogEntry as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LogEntry - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LogRequest {
    #[serde(rename = "log")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log: Option<models::LogRequestLog>,

}





impl LogRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> LogRequest {
        LogRequest {
            log: None,
        }
    }
}

/// Converts the LogRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for LogRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping log in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LogRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LogRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub log: Vec<models::LogRequestLog>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LogRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "log" => intermediate_rep.log.push(<models::LogRequestLog as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing LogRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LogRequest {
            log: intermediate_rep.log.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LogRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<LogRequest>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LogRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LogRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<LogRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LogRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LogRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LogRequestLog {
    #[serde(rename = "logs_array")]
    pub logs_array: Vec<models::LogEntry>,

}





impl LogRequestLog {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(logs_array: Vec<models::LogEntry>, ) -> LogRequestLog {
        LogRequestLog {
            logs_array,
        }
    }
}

/// Converts the LogRequestLog value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for LogRequestLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping logs_array in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LogRequestLog value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LogRequestLog {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub logs_array: Vec<Vec<models::LogEntry>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LogRequestLog".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "logs_array" => return std::result::Result::Err("Parsing a container in this style is not supported in LogRequestLog".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing LogRequestLog".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LogRequestLog {
            logs_array: intermediate_rep.logs_array.into_iter().next().ok_or_else(|| "logs_array missing in LogRequestLog".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LogRequestLog> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<LogRequestLog>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LogRequestLog>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LogRequestLog - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<LogRequestLog> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LogRequestLog as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LogRequestLog - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PostLog200Response {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<i32>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

}





impl PostLog200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> PostLog200Response {
        PostLog200Response {
            status: None,
            message: None,
        }
    }
}

/// Converts the PostLog200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PostLog200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PostLog200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PostLog200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub status: Vec<i32>,
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PostLog200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PostLog200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PostLog200Response {
            status: intermediate_rep.status.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PostLog200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PostLog200Response>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PostLog200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PostLog200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<PostLog200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PostLog200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PostLog200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PostLog400Response {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<i32>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

}





impl PostLog400Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> PostLog400Response {
        PostLog400Response {
            status: None,
            message: None,
        }
    }
}

/// Converts the PostLog400Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PostLog400Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PostLog400Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PostLog400Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub status: Vec<i32>,
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PostLog400Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PostLog400Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PostLog400Response {
            status: intermediate_rep.status.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PostLog400Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PostLog400Response>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PostLog400Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PostLog400Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<PostLog400Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PostLog400Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PostLog400Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}



