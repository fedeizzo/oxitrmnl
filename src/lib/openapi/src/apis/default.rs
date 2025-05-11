use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetDisplayResponse {
    /// Successful retrieval of display information.
    Status200_SuccessfulRetrievalOfDisplayInformation
    (models::DisplayInfo)
    ,
    /// Device not found or internal server error.
    Status500_DeviceNotFoundOrInternalServerError
    (models::GetDisplay500Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetSetupResponse {
    /// Successful device information response.
    Status200_SuccessfulDeviceInformationResponse
    (models::DeviceInfo)
    ,
    /// Missing ID header.
    Status404_MissingIDHeader
    (models::DeviceInfo)
    ,
    /// Internal server error.
    Status500_InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostLogResponse {
    /// Successful log reception.
    Status200_SuccessfulLogReception
    (models::PostLog200Response)
    ,
    /// No device found or created.
    Status400_NoDeviceFoundOrCreated
    (models::PostLog400Response)
    ,
    /// Internal server error.
    Status500_InternalServerError
    (models::ErrorResponse)
}


/// Default
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Default<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Retrieve display information for a device..
    ///
    /// GetDisplay - GET /api/display
    async fn get_display(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::GetDisplayHeaderParams,
    ) -> Result<GetDisplayResponse, E>;

    /// Set up or retrieve device information..
    ///
    /// GetSetup - GET /api/setup
    async fn get_setup(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::GetSetupHeaderParams,
    ) -> Result<GetSetupResponse, E>;

    /// Receive and process device logs..
    ///
    /// PostLog - POST /api/log
    async fn post_log(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::PostLogHeaderParams,
            body: &models::LogRequest,
    ) -> Result<PostLogResponse, E>;
}
