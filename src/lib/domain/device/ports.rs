use std::future::Future;

use super::models::device::{CreateDeviceRequest, Device, DeviceError, GetDeviceRequest};

pub trait DeviceRepository {
    fn create_device(
        &self,
        request: &CreateDeviceRequest,
    ) -> impl Future<Output = Result<Device, DeviceError>> + Send;
    fn get_device(
        &self,
        request: &GetDeviceRequest,
    ) -> impl Future<Output = Result<Device, DeviceError>> + Send;
}

pub trait DeviceService {
    fn create_device(
        &self,
        request: &CreateDeviceRequest,
    ) -> impl Future<Output = Result<Device, DeviceError>> + Send;
    fn get_device(
        &self,
        request: &GetDeviceRequest,
    ) -> impl Future<Output = Result<Device, DeviceError>> + Send;
}
