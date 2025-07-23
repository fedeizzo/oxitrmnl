use axum::http::Method;
use axum_extra::extract::{CookieJar, Host};
use log::error;
use oxitrmnl_openapi::{
    apis::{
        default::{Default, GetDisplayResponse, GetSetupResponse, PostLogResponse},
        ErrorHandler,
    },
    models::{self, GetDisplay500Response},
    types::Nullable,
};

use crate::domain::device::{
    models::device::{CreateDeviceRequest, MacAddress},
    ports::DeviceService,
};

#[derive(Clone)]
pub struct HttpServer<S: DeviceService> {
    device_service: S,
}

impl<S: DeviceService> AsRef<HttpServer<S>> for HttpServer<S> {
    fn as_ref(&self) -> &HttpServer<S> {
        return self;
    }
}

impl<S: DeviceService> HttpServer<S> {
    pub fn new(device_service: S) -> HttpServer<S> {
        HttpServer { device_service }
    }
}

impl<S: DeviceService> ErrorHandler for HttpServer<S> {}

#[async_trait::async_trait]
impl<S: DeviceService + std::marker::Sync> Default for HttpServer<S> {
    async fn get_display(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        header_params: &models::GetDisplayHeaderParams,
    ) -> Result<GetDisplayResponse, ()> {
        return Ok(
            GetDisplayResponse::Status500_DeviceNotFoundOrInternalServerError(
                GetDisplay500Response::new(),
            ),
        );
    }

    async fn get_setup(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        header_params: &models::GetSetupHeaderParams,
    ) -> Result<GetSetupResponse, ()> {
        if header_params.id.is_none() {
            error!("missing mac address");
            return Ok(GetSetupResponse::Status404_MissingIDHeader(
                models::DeviceInfo::new(),
            ));
        }
        let id = header_params.id.as_ref().unwrap();

        let mac_address = MacAddress::new(&id);
        if mac_address.is_err() {
            error!("invalid mac address");
            return Ok(GetSetupResponse::Status500_InternalServerError(
                models::ErrorResponse {
                    status: None,
                    message: Some(
                        "the provided it cannot be parsed into a valid mac address".to_string(),
                    ),
                    error: Some(Nullable::Present("invalid mac address".to_string())),
                },
            ));
        }

        match self
            .device_service
            .create_device(&CreateDeviceRequest::new(
                "".to_string(),
                mac_address.unwrap(),
            ))
            .await
        {
            Ok(device) => {
                return Ok(
                    GetSetupResponse::Status200_SuccessfulDeviceInformationResponse(
                        models::DeviceInfo {
                            api_key: Some(Nullable::Present(device.api_key)),
                            friendly_id: Some(Nullable::Present(device.friendly_name)),
                            image_url: None,
                            filename: None,
                            message: None,
                            reset_firmware: None,
                            error: None,
                            status: None,
                        },
                    ),
                );
            }
            Err(err) => {
                error!("cannot setup new device: {}", err);
                return Ok(GetSetupResponse::Status500_InternalServerError(
                    models::ErrorResponse {
                        status: None,
                        message: Some("cannot setup new device".to_string()),
                        error: Some(Nullable::Present(format!("{}", err))),
                    },
                ));
            }
        }
    }

    async fn post_log(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        header_params: &models::PostLogHeaderParams,
        body: &models::LogRequest,
    ) -> Result<PostLogResponse, ()> {
        todo!()
    }
    // API implementation goes here
}
